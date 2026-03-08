use crate::codegen::ir::IrInstruction;
use crate::{
    ast::Stmt,
    codegen::{
        expr::{emit_expr, infer_expr_type},
        module::{CodegenError, FuncGen, FunctionSig},
    },
};
use std::collections::HashMap;
use wasm_encoder::*;

/// Emits instructions for one statement.
pub fn emit_stmt(
    stmt: &Stmt,
    r#gen: &mut FuncGen,
    funcs: &HashMap<String, FunctionSig>,
) -> Result<(), CodegenError> {
    match stmt {
        Stmt::Let { name, value, .. } => {
            let ty = r#gen
                .local_map
                .get(name)
                .ok_or_else(|| CodegenError::UnknownLocal {
                    name: name.clone(),
                })?
                .1
                .clone();
            let produced = match ty {
                crate::ast::Type::Unit => false,
                _ => true,
            };

            emit_expr(value, r#gen, funcs)?;

            let idx = match r#gen.local_map.get(name) {
                Some((idx, _)) => *idx,
                None => {
                    return Err(CodegenError::UnknownLocal {
                        name: name.clone(),
                    })
                }
            };
            if produced {
                r#gen.instructions.push(IrInstruction::LocalSet(idx));
            } else {
                return Err(CodegenError::UnsupportedType(
                    "unit-like let declaration".to_string(),
                ));
            }
        }

        Stmt::Expr(expr) => {
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
                    // If a return type exists, return a default value.
                    if r#gen.has_return {
                        match r#gen.return_type {
                            crate::ast::Type::Int => r#gen.instructions.push(IrInstruction::I64Const(0)),
                            crate::ast::Type::Float => {
                                r#gen.instructions.push(IrInstruction::F64Const(0.0));
                            }
                            crate::ast::Type::String => {
                                r#gen.instructions.push(IrInstruction::I32Const(0));
                            }
                            _ => r#gen.instructions.push(IrInstruction::I32Const(0)),
                        }
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
            let cond_ty = infer_expr_type(cond, r#gen, funcs)?;
            if cond_ty != crate::ast::Type::Bool {
                return Err(CodegenError::UnsupportedType(
                    "if condition must be Bool".to_string(),
                ));
            }

            let _ = emit_expr(cond, r#gen, funcs)?;
            // cond is Bool with expected type i32 in this backend shape.
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
            let cond_ty = infer_expr_type(cond, r#gen, funcs)?;
            if cond_ty != crate::ast::Type::Bool {
                return Err(CodegenError::UnsupportedType(
                    "while condition must be Bool".to_string(),
                ));
            }

            r#gen
                .instructions
                .push(IrInstruction::Block(BlockType::Empty));
            r#gen
                .instructions
                .push(IrInstruction::Loop(BlockType::Empty));

            let _ = emit_expr(cond, r#gen, funcs)?;
            // cond is Bool encoded as i32. Exit loop on cond == 0.
            r#gen.instructions.push(IrInstruction::I32Eqz);
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
