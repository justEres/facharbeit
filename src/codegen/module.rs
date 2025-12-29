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
    functions: FunctionSection,
    codes: CodeSection,
    exports: ExportSection,

    func_indices: HashMap<String, u32>,
    next_type_index: u32,
}

impl ModuleGen {
    pub fn new() -> Self {
        Self {
            module: Module::new(),
            types: TypeSection::new(),
            functions: FunctionSection::new(),
            codes: CodeSection::new(),
            exports: ExportSection::new(),
            func_indices: HashMap::new(),
            next_type_index: 0,
        }
    }

    pub fn finish(mut self) -> Vec<u8> {
        self.module.section(&self.types);
        self.module.section(&self.functions);
        self.module.section(&self.exports);
        self.module.section(&self.codes);
        self.module.finish()
    }

    pub fn declare_function(&mut self, func: &FunctionDecl) {
        let type_index = self.next_type_index;
        self.next_type_index += 1;

        self.types
            .ty()
            .function(vec![ValType::I64; func.params.len()], [ValType::I64]);

        let idx = self.func_indices.len() as u32;
        self.func_indices.insert(func.name.clone(), idx);

        self.functions.function(type_index);
        self.exports.export(&func.name, ExportKind::Func, idx);
    }

    pub fn emit_function(&mut self, func: &FunctionDecl) {
        let mut r#gen = FuncGen {
            locals: Vec::new(),
            local_map: HashMap::new(),
            instructions: Vec::new(),
        };

        for (i, name) in func.params.iter().enumerate() {
            r#gen.local_map.insert(name.clone(), i as u32);
        }

        for stmt in &func.body {
            emit_stmt(stmt, &mut r#gen, &self.func_indices);
        }

        r#gen.instructions.push(IrInstruction::I64Const(0));
        r#gen.instructions.push(IrInstruction::Return);
    

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
}
