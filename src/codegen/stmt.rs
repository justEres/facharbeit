use crate::{
    ast::Stmt,
    codegen::{expr::emit_expr, module::FuncGen},
};
use std::collections::HashMap;
use wasm_encoder::*;

pub fn emit_stmt(stmt: &Stmt, r#gen: &mut FuncGen, funcs: &HashMap<String, u32>) {
    match stmt {
        Stmt::Let { name, value } => {
            let idx = r#gen.local_map.len() as u32;
            r#gen.locals.push(wasm_encoder::ValType::I64);
            r#gen.local_map.insert(name.clone(), idx);

            emit_expr(value, r#gen, funcs);
            r#gen.body.instruction(&Instruction::LocalSet(idx));
        }

        Stmt::Expr(expr) => {
            emit_expr(expr, r#gen, funcs);
            r#gen.body.instruction(&Instruction::Drop); // why the drop?
        }

        Stmt::Return(expr) => {
            emit_expr(expr, r#gen, funcs);
            r#gen.body.instruction(&Instruction::Return);
        }

        Stmt::If {
            cond,
            then_block,
            else_block,
        } => {
            emit_expr(cond, r#gen, funcs);
            r#gen.body.instruction(&Instruction::If(BlockType::Empty)); // what does empty do?

            for s in then_block {
                emit_stmt(s, r#gen, funcs);
            }

            if !else_block.is_empty() {
                r#gen.body.instruction(&Instruction::Else);
                for s in else_block {
                    emit_stmt(stmt, r#gen, funcs);
                }
            }

            r#gen.body.instruction(&Instruction::End);
        }

        Stmt::While { cond, body } => {
            r#gen
                .body
                .instruction(&Instruction::Block(BlockType::Empty));
            r#gen.body.instruction(&Instruction::Loop(BlockType::Empty));

            emit_expr(cond, r#gen, funcs);
            r#gen.body.instruction(&Instruction::I64Eqz);
            r#gen.body.instruction(&Instruction::BrIf(1));

            for s in body {
                emit_stmt(s, r#gen, funcs);
            }

            r#gen.body.instruction(&Instruction::Br(0));
            r#gen.body.instruction(&Instruction::End);
            r#gen.body.instruction(&Instruction::End);
        }
    }
}
