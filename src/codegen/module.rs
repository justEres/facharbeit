use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::codegen::ir::IrInstruction;
use wasm_encoder::*;

use crate::{ast::FunctionDecl, codegen::stmt::emit_stmt};

/// Code generation failures that can occur after parsing.
#[derive(Debug)]
pub enum CodegenError {
    /// Function name is declared more than once in one module.
    DuplicateFunction { name: String },
    /// A referenced local variable was not declared.
    UnknownLocal { name: String },
    /// A call targets a function that was never declared/imported.
    UnknownFunction { name: String },
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
        }
    }
}

impl Error for CodegenError {}

/// Stateful WebAssembly module builder for language functions.
pub struct ModuleGen {
    module: Module,
    types: TypeSection,
    imports: ImportSection,
    functions: FunctionSection,
    codes: CodeSection,
    exports: ExportSection,

    func_indices: HashMap<String, (u32, bool)>,
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

    /// Registers built-in host functions (currently `print`).
    pub fn init_with_host_functions(mut self) -> Self {
        // Register host imports (currently only print).
        self.add_print_import();
        self
    }

    fn add_print_import(&mut self) {
        // (i64) -> ()
        let type_index = self.next_type_index;
        self.next_type_index += 1;
        self.types.ty().function(vec![ValType::I64], Vec::new());

        // Import as env.print_i64.
        self.imports
            .import("env", "print_i64", EntityType::Function(type_index));

        // Expose this import as `print` in the compiler function table.
        let idx = self.next_func_index;
        self.next_func_index += 1;
        self.func_indices.insert("print".to_string(), (idx, false));
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
    pub fn declare_function(&mut self, func: &FunctionDecl) -> Result<(), CodegenError> {
        if self.func_indices.contains_key(&func.name) {
            return Err(CodegenError::DuplicateFunction {
                name: func.name.clone(),
            });
        }

        let type_index = self.next_type_index;
        self.next_type_index += 1;

        let params = vec![ValType::I64; func.params.len()];
        let results = if func.return_type.is_some() {
            vec![ValType::I64]
        } else {
            Vec::new()
        };

        self.types.ty().function(params, results);

        let idx = self.func_indices.len() as u32;
        self.func_indices
            .insert(func.name.clone(), (idx, func.return_type.is_some()));

        self.functions.function(type_index);
        self.exports.export(&func.name, ExportKind::Func, idx);
        Ok(())
    }

    /// Emits one function body into the code section.
    pub fn emit_function(&mut self, func: &FunctionDecl) -> Result<(), CodegenError> {
        let mut r#gen = FuncGen {
            locals: Vec::new(),
            local_map: HashMap::new(),
            instructions: Vec::new(),
            has_return: func.return_type.is_some(),
        };

        for (i, name) in func.params.iter().enumerate() {
            r#gen.local_map.insert(name.clone(), i as u32);
        }

        for stmt in &func.body {
            emit_stmt(stmt, &mut r#gen, &self.func_indices)?;
        }

        // If the function has a return type but no explicit return was emitted,
        // provide a default 0 return so the function type matches. If no return
        // type is declared, don't emit a return value.
        if func.return_type.is_some() {
            r#gen.instructions.push(IrInstruction::I64Const(0));
            r#gen.instructions.push(IrInstruction::Return);
        }

        let mut local_groups = Vec::new();
        for ty in r#gen.locals {
            local_groups.push((1, ty));
        }

        let mut wasm_func = Function::new(local_groups);

        for instr in r#gen.instructions {
            wasm_func.instruction(&instr.to_wasm());
        }

        wasm_func.instruction(&Instruction::End);

        self.codes.function(&wasm_func);
        Ok(())
    }
}

/// Per-function code generation state.
pub struct FuncGen {
    /// Additional local declarations (params are implicit locals 0..n).
    pub locals: Vec<ValType>,
    /// Symbol table for locals and parameters.
    pub local_map: HashMap<String, u32>,
    // Instruction list for one function body.
    pub instructions: Vec<IrInstruction>,
    /// Whether the current function expects a return value.
    pub has_return: bool,
}
