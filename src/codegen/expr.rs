use std::collections::HashMap;

use crate::{
    ast::{BinOp, Expr},
    codegen::ir::IrInstruction,
    codegen::module::FuncGen,
};

pub fn emit_expr(expr: &Expr, r#gen: &mut FuncGen, funcs: &HashMap<String, (u32, bool)>) -> bool {
    match expr {
        Expr::Int(v) => {
            r#gen.instructions.push(IrInstruction::I64Const(*v));
            true
        }
        Expr::Local(name) => {
            let idx = r#gen.local_map[name];
            r#gen.instructions.push(IrInstruction::LocalGet(idx));
            true
        }
        Expr::Binary { op, left, right } => {
            let _ = emit_expr(left, r#gen, funcs);
            let _ = emit_expr(right, r#gen, funcs);

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

            true
        }
        Expr::Call { name, args } => {
            for arg in args {
                let _ = emit_expr(arg, r#gen, funcs);
            }
            let (idx, has_ret) = funcs[name];
            r#gen.instructions.push(IrInstruction::Call(idx));
            has_ret
        }
    }
}
