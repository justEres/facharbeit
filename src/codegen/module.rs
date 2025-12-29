use std::collections::HashMap;

use wasm_encoder::*;
use crate::codegen::ir::IrInstruction;

use crate::{
    ast::FunctionDecl,
    codegen::stmt::emit_stmt,
};

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

    pub fn init_with_host_functions(mut self) -> Self {
        // register host imports (print)
        self.add_print_import();
        self
    }

    fn add_print_import(&mut self) {
        // create a function type for (i64) -> ()
        let type_index = self.next_type_index;
        self.next_type_index += 1;
        self.types.ty().function(vec![ValType::I64], Vec::new());

        // import under module "env" with name "print_i64"
        self.imports
            .import("env", "print_i64", EntityType::Function(type_index));

    // assign function index for the imported function and expose under name "print"
    let idx = self.next_func_index;
    self.next_func_index += 1;
    // imported `print` has no return value
    self.func_indices.insert("print".to_string(), (idx, false));
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.module.section(&self.types);
        self.module.section(&self.imports);
        self.module.section(&self.functions);
        self.module.section(&self.exports);
        self.module.section(&self.codes);
        self.module.finish()
    }

    pub fn declare_function(&mut self, func: &FunctionDecl) {
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
    }

    pub fn emit_function(&mut self, func: &FunctionDecl) {
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
            emit_stmt(stmt, &mut r#gen, &self.func_indices);
        }

        // If the function has a return type but no explicit return was emitted,
        // provide a default 0 return so the function type matches. If no return
        // type is declared, don't emit a return value.
        if func.return_type.is_some() {
            r#gen.instructions.push(IrInstruction::I64Const(0));
            r#gen.instructions.push(IrInstruction::Return);
        }
    

        let mut local_groups = Vec::new();
        for ty in r#gen.locals{
            local_groups.push((1,ty));
        }

        let mut wasm_func = Function::new(local_groups);

        for instr in r#gen.instructions{
            wasm_func.instruction(&instr.to_wasm());
        }

        wasm_func.instruction(&Instruction::End);

                

        self.codes.function(&wasm_func);


    }
}

pub struct FuncGen {
    pub locals: Vec<ValType>,
    pub local_map: HashMap<String, u32>,
    pub instructions: Vec<IrInstruction>, //TODO: Custom Ir to first declare locals and later generate instructions
    pub has_return: bool,
}
