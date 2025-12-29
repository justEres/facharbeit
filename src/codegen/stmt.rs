use crate::{
    ast::Stmt,
    codegen::{expr::emit_expr, module::FuncGen},
};
use std::collections::HashMap;
use wasm_encoder::*;
use crate::codegen::ir::IrInstruction;

pub fn emit_stmt(stmt: &Stmt, r#gen: &mut FuncGen, funcs: &HashMap<String, (u32, bool)>) {
    match stmt {
        Stmt::Let { name, value } => {
            let idx = r#gen.local_map.len() as u32;
            r#gen.locals.push(wasm_encoder::ValType::I64);
            r#gen.local_map.insert(name.clone(), idx);

            emit_expr(value, r#gen, funcs);
            r#gen.instructions.push(IrInstruction::LocalSet(idx));
        }

        Stmt::Expr(expr) => {
            // Only drop the expression result if the expression actually left
            // a value on the stack. emit_expr now returns true when it
            // pushed a value.
            let produced = emit_expr(expr, r#gen, funcs);
            if produced {
                r#gen.instructions.push(IrInstruction::Drop);
            }
        }

        Stmt::Return(expr_opt) => {
            match expr_opt {
                Some(expr) => {
                    emit_expr(expr, r#gen, funcs);
                }
                None => {
                    // If the function declares a return type, supply a default 0 value.
                    if r#gen.has_return {
                        r#gen.instructions.push(IrInstruction::I64Const(0));
                    }
                    // otherwise return with no value
                }
            }

            r#gen.instructions.push(IrInstruction::Return);
        }

        Stmt::If {
            cond,
            then_block,
            else_block,
        } => {
            emit_expr(cond, r#gen, funcs);
            // emit_expr produces i64 for comparisons and integers; Wasm `if` expects an i32
            // convert i64 (0/1) -> i32 boolean: i64.eqz -> i32 (1 if zero), then i32.eqz to invert
            r#gen.instructions.push(IrInstruction::I64Eqz);
            r#gen.instructions.push(IrInstruction::I32Eqz);
            r#gen.instructions.push(IrInstruction::If(BlockType::Empty)); // what does empty do?

            for s in then_block {
                emit_stmt(s, r#gen, funcs);
            }

            if !else_block.is_empty() {
                r#gen.instructions.push(IrInstruction::Else);
                for s in else_block {
                    emit_stmt(s, r#gen, funcs);
                }
            }

            r#gen.instructions.push(IrInstruction::End);
        }

        Stmt::While { cond, body } => {
            r#gen
                .instructions
                .push(IrInstruction::Block(BlockType::Empty));
            r#gen.instructions.push(IrInstruction::Loop(BlockType::Empty));

            emit_expr(cond, r#gen, funcs);
            r#gen.instructions.push(IrInstruction::I64Eqz);
            r#gen.instructions.push(IrInstruction::BrIf(1));

            for s in body {
                emit_stmt(s, r#gen, funcs);
            }

            r#gen.instructions.push(IrInstruction::Br(0));
            r#gen.instructions.push(IrInstruction::End);
            r#gen.instructions.push(IrInstruction::End);
        }
    }
}
