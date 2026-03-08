use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ast::Type;
use crate::ast::FunctionDecl;
use crate::codegen::ir::IrInstruction;
use crate::typing::TypedFunction;
use eres_abi::{AbiType, HostFunction};
use wasm_encoder::*;

use crate::codegen::stmt::emit_stmt;

/// Code generation failures that can occur after parsing.
#[derive(Debug)]
pub enum CodegenError {
    /// Function name is declared more than once in one module.
    DuplicateFunction { name: String },
    /// A referenced local variable was not declared.
    UnknownLocal { name: String },
    /// A call targets a function that was never declared/imported.
    UnknownFunction { name: String },
    /// Type is not supported by the current backend.
    UnsupportedType(String),
}

impl Display for CodegenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CodegenError::DuplicateFunction { name } => {
                write!(f, "duplicate function declaration: `{}`", name)
            }
            CodegenError::UnknownLocal { name } => write!(f, "unknown local variable: `{}`", name),
            CodegenError::UnknownFunction { name } => {
                write!(f, "unknown function call target: `{}`", name)
            }
            CodegenError::UnsupportedType(ty) => write!(f, "unsupported type in current codegen backend: {}", ty),
        }
    }
}

impl Error for CodegenError {}

/// Stateful WebAssembly module builder for language functions.
pub type FunctionSig = (u32, Vec<Type>, Type);

pub struct ModuleGen {
    module: Module,
    types: TypeSection,
    imports: ImportSection,
    functions: FunctionSection,
    codes: CodeSection,
    exports: ExportSection,

    func_indices: HashMap<String, FunctionSig>,
    next_type_index: u32,
    next_func_index: u32,
}

impl ModuleGen {
    /// Creates an empty module generator.
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            types: TypeSection::new(),
            imports: ImportSection::new(),
            functions: FunctionSection::new(),
            codes: CodeSection::new(),
            exports: ExportSection::new(),
            func_indices: HashMap::new(),
            next_type_index: 0,
            next_func_index: 0,
        }
    }

    /// Registers host functions that are available globally to eres code.
    pub fn init_with_host_functions(mut self, hosts: &[HostFunction]) -> Result<Self, CodegenError> {
        for host in hosts {
            self.add_host_import(host)?;
        }
        Ok(self)
    }

    fn add_host_import(&mut self, host: &HostFunction) -> Result<(), CodegenError> {
        if self.func_indices.contains_key(host.name) {
            return Err(CodegenError::DuplicateFunction {
                name: host.name.to_string(),
            });
        }

        let type_index = self.next_type_index;
        self.next_type_index += 1;
        let params = host
            .params
            .iter()
            .map(abi_to_wasm_encoder_val_type)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| {
                CodegenError::UnsupportedType(format!("unsupported host parameter in `{}`", host.name))
            })?;
        let results = match &host.result {
            AbiType::Unit => Vec::new(),
            other => vec![abi_to_wasm_encoder_val_type(&other).ok_or_else(|| {
                CodegenError::UnsupportedType(format!("unsupported host return type in `{}`", host.name))
            })?],
        };
        self.types.ty().function(params, results);

        self.imports
            .import("env", host.name, EntityType::Function(type_index));

        let idx = self.next_func_index;
        self.next_func_index += 1;
        self.func_indices.insert(
            host.name.to_string(),
            (
                idx,
                host.params
                    .iter()
                    .map(abi_to_ast_type)
                    .collect(),
                abi_to_ast_type(&host.result),
            ),
        );
        Ok(())
    }

    /// Finalizes sections and returns the encoded wasm module bytes.
    pub fn finish(mut self) -> Vec<u8> {
        self.module.section(&self.types);
        self.module.section(&self.imports);
        self.module.section(&self.functions);
        self.module.section(&self.exports);
        self.module.section(&self.codes);
        self.module.finish()
    }

    /// Reserves function signature/index metadata before emitting bodies.
    pub fn declare_function(
        &mut self,
        func: &FunctionDecl,
        typed: &TypedFunction,
    ) -> Result<(), CodegenError> {
        if self.func_indices.contains_key(&func.name) {
            return Err(CodegenError::DuplicateFunction {
                name: func.name.clone(),
            });
        }

        let param_tys: Vec<ValType> = typed
            .params
            .iter()
            .map(|p| wasm_val_type_for(&p.ty))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| CodegenError::UnsupportedType(format!(
                "unsupported parameter in function `{}`",
                func.name
            )))?;

        let results = if func.return_type == Type::Unit {
            Vec::new()
        } else {
            vec![wasm_val_type_for(&func.return_type).ok_or_else(|| {
                CodegenError::UnsupportedType(format!(
                    "unsupported return type for function `{}`",
                    func.name
                ))
            })?]
        };

        let type_index = self.next_type_index;
        self.next_type_index += 1;
        self.types.ty().function(param_tys, results);

        let idx = self.next_func_index;
        self.next_func_index += 1;
        self.func_indices
            .insert(
                func.name.clone(),
                (
                    idx,
                    typed.params.iter().map(|p| p.ty.clone()).collect(),
                    func.return_type.clone(),
                ),
            );

        self.functions.function(type_index);
        self.exports.export(&func.name, ExportKind::Func, idx);
        Ok(())
    }

    /// Emits one function body into the code section.
    pub fn emit_function(
        &mut self,
        func: &FunctionDecl,
        typed: &TypedFunction,
    ) -> Result<(), CodegenError> {
        let mut cg = FuncGen {
            locals: Vec::new(),
            local_map: HashMap::new(),
            instructions: Vec::new(),
            has_return: func.return_type != Type::Unit,
            return_type: func.return_type.clone(),
        };

        for (i, param) in typed.params.iter().enumerate() {
            cg.local_map
                .insert(param.name.clone(), (i as u32, param.ty.clone()));
        }

        for (name, ty) in &typed.locals {
            let wasm_ty = wasm_val_type_for(ty).ok_or_else(|| {
                CodegenError::UnsupportedType(format!("unsupported local `{}`", name))
            })?;
            let idx = cg.local_map.len() as u32;
            cg.local_map
                .insert(name.clone(), (idx, ty.clone()));
            cg.locals.push(wasm_ty);
        }

        for stmt in &func.body {
            emit_stmt(stmt, &mut cg, &self.func_indices)?;
        }

        if func.return_type != Type::Unit {
            match &func.return_type {
                Type::Int => cg.instructions.push(IrInstruction::I64Const(0)),
                Type::Float => cg.instructions.push(IrInstruction::F64Const(0.0)),
                Type::Bool
                | Type::String
                | Type::Ref(_)
                | Type::List(_)
                | Type::Tuple(_)
                | Type::Named(_) => {
                    cg.instructions.push(IrInstruction::I32Const(0))
                }
                Type::Unit | Type::Function(_, _) => {}
            }
            cg.instructions.push(IrInstruction::Return);
        }

        let mut local_groups = Vec::new();
        for ty in cg.locals {
            local_groups.push((1, ty));
        }

        let mut wasm_func = Function::new(local_groups);

        for instr in cg.instructions {
            wasm_func.instruction(&instr.to_wasm());
        }

        wasm_func.instruction(&Instruction::End);

        self.codes.function(&wasm_func);
        Ok(())
    }
}

fn abi_to_ast_type(ty: &AbiType) -> Type {
    match ty {
        AbiType::Int => Type::Int,
        AbiType::Float => Type::Float,
        AbiType::Bool => Type::Bool,
        AbiType::String => Type::String,
        AbiType::List(inner) => Type::List(Box::new(abi_to_ast_type(inner))),
        AbiType::Tuple(elements) => Type::Tuple(elements.iter().map(abi_to_ast_type).collect()),
        AbiType::Named(named) => Type::Named(named.name.clone()),
        AbiType::Unit => Type::Unit,
    }
}

fn abi_to_wasm_encoder_val_type(ty: &AbiType) -> Option<ValType> {
    match ty {
        AbiType::Int => Some(ValType::I64),
        AbiType::Float => Some(ValType::F64),
        AbiType::Bool => Some(ValType::I32),
        AbiType::String | AbiType::List(_) | AbiType::Tuple(_) | AbiType::Named(_) => {
            Some(ValType::I32)
        }
        AbiType::Unit => None,
    }
}

fn wasm_val_type_for(ty: &Type) -> Option<ValType> {
    match ty {
        Type::Int => Some(ValType::I64),
        Type::Float => Some(ValType::F64),
        Type::Bool => Some(ValType::I32),
        Type::String | Type::Ref(_) | Type::List(_) | Type::Tuple(_) | Type::Named(_) => {
            Some(ValType::I32)
        }
        Type::Function(_, _) => None,
        Type::Unit => None,
    }
}

/// Per-function code generation state.
pub struct FuncGen {
    /// Additional local declarations (params are implicit locals 0..n).
    pub locals: Vec<ValType>,
    /// Symbol table for locals and parameters -> (index, type).
    pub local_map: HashMap<String, (u32, Type)>,
    // Instruction list for one function body.
    pub instructions: Vec<IrInstruction>,
    /// Whether the current function expects a return value.
    pub has_return: bool,
    /// Current function return type.
    pub return_type: Type,
}
