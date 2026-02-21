use std::collections::HashMap;

use crate::{
    ast::{BinOp, Expr},
    codegen::ir::IrInstruction,
    codegen::module::{CodegenError, FuncGen},
};

/// Emits instructions for an expression and returns whether it leaves a stack value.
pub fn emit_expr(
    expr: &Expr,
    r#gen: &mut FuncGen,
    funcs: &HashMap<String, (u32, bool)>,
) -> Result<bool, CodegenError> {
    match expr {
        Expr::Int(v) => {
            r#gen.instructions.push(IrInstruction::I64Const(*v));
            Ok(true)
        }
        Expr::Local(name) => {
            let idx = *r#gen
                .local_map
                .get(name)
                .ok_or_else(|| CodegenError::UnknownLocal { name: name.clone() })?;
            r#gen.instructions.push(IrInstruction::LocalGet(idx));
            Ok(true)
        }
        Expr::Binary { op, left, right } => {
            let _ = emit_expr(left, r#gen, funcs)?;
            let _ = emit_expr(right, r#gen, funcs)?;

            match op {
                BinOp::Add => r#gen.instructions.push(IrInstruction::I64Add),
                BinOp::Sub => r#gen.instructions.push(IrInstruction::I64Sub),
                BinOp::Mul => r#gen.instructions.push(IrInstruction::I64Mul),
                BinOp::Div => r#gen.instructions.push(IrInstruction::I64DivS),
                BinOp::Eq => {
                    r#gen.instructions.push(IrInstruction::I64Eq);
                    // Keep expression results as i64 across the compiler.
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                BinOp::Lt => {
                    r#gen.instructions.push(IrInstruction::I64LtS);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                BinOp::Gt => {
                    r#gen.instructions.push(IrInstruction::I64GtS);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                // a != b  =>  !(a == b)
                BinOp::NotEq => {
                    r#gen.instructions.push(IrInstruction::I64Eq);
                    r#gen.instructions.push(IrInstruction::I32Eqz);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                // a <= b  =>  !(a > b)
                BinOp::Le => {
                    r#gen.instructions.push(IrInstruction::I64GtS);
                    r#gen.instructions.push(IrInstruction::I32Eqz);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                // a >= b  =>  !(a < b)
                BinOp::Ge => {
                    r#gen.instructions.push(IrInstruction::I64LtS);
                    r#gen.instructions.push(IrInstruction::I32Eqz);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
            }

            Ok(true)
        }
        Expr::Call { name, args } => {
            for arg in args {
                let _ = emit_expr(arg, r#gen, funcs)?;
            }
            let (idx, has_ret) =
                *funcs
                    .get(name)
                    .ok_or_else(|| CodegenError::UnknownFunction { name: name.clone() })?;
            r#gen.instructions.push(IrInstruction::Call(idx));
            Ok(has_ret)
        }
    }
}
