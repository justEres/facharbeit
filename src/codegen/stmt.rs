use crate::codegen::ir::IrInstruction;
use crate::{
    ast::Stmt,
    codegen::{
        expr::emit_expr,
        module::{CodegenError, FuncGen},
    },
};
use std::collections::HashMap;
use wasm_encoder::*;

pub fn emit_stmt(
    stmt: &Stmt,
    r#gen: &mut FuncGen,
    funcs: &HashMap<String, (u32, bool)>,
) -> Result<(), CodegenError> {
    match stmt {
        Stmt::Let { name, value } => {
            let idx = r#gen.local_map.len() as u32;
            r#gen.locals.push(wasm_encoder::ValType::I64);
            r#gen.local_map.insert(name.clone(), idx);

            let _ = emit_expr(value, r#gen, funcs)?;
            r#gen.instructions.push(IrInstruction::LocalSet(idx));
        }

        Stmt::Expr(expr) => {
            // Drop only if this expression produced a stack value.
            let produced = emit_expr(expr, r#gen, funcs)?;
            if produced {
                r#gen.instructions.push(IrInstruction::Drop);
            }
        }

        Stmt::Return(expr_opt) => {
            match expr_opt {
                Some(expr) => {
                    let _ = emit_expr(expr, r#gen, funcs)?;
                }
                None => {
                    // If a return type exists, return a default zero value.
                    if r#gen.has_return {
                        r#gen.instructions.push(IrInstruction::I64Const(0));
                    }
                }
            }

            r#gen.instructions.push(IrInstruction::Return);
        }

        Stmt::If {
            cond,
            then_block,
            else_block,
        } => {
            let _ = emit_expr(cond, r#gen, funcs)?;
            // emit_expr leaves i64 values, while wasm `if` expects i32.
            // Convert i64 truthy/falsey into an i32 condition.
            r#gen.instructions.push(IrInstruction::I64Eqz);
            r#gen.instructions.push(IrInstruction::I32Eqz);
            r#gen.instructions.push(IrInstruction::If(BlockType::Empty));

            for s in then_block {
                emit_stmt(s, r#gen, funcs)?;
            }

            if !else_block.is_empty() {
                r#gen.instructions.push(IrInstruction::Else);
                for s in else_block {
                    emit_stmt(s, r#gen, funcs)?;
                }
            }

            r#gen.instructions.push(IrInstruction::End);
        }

        Stmt::While { cond, body } => {
            r#gen
                .instructions
                .push(IrInstruction::Block(BlockType::Empty));
            r#gen
                .instructions
                .push(IrInstruction::Loop(BlockType::Empty));

            let _ = emit_expr(cond, r#gen, funcs)?;
            r#gen.instructions.push(IrInstruction::I64Eqz);
            r#gen.instructions.push(IrInstruction::BrIf(1));

            for s in body {
                emit_stmt(s, r#gen, funcs)?;
            }

            r#gen.instructions.push(IrInstruction::Br(0));
            r#gen.instructions.push(IrInstruction::End);
            r#gen.instructions.push(IrInstruction::End);
        }
    }
    Ok(())
}
