use std::collections::HashMap;



use crate::{
    ast::{BinOp, Expr},
    codegen::module::FuncGen,
    codegen::ir::IrInstruction,
};

pub fn emit_expr(
    expr: &Expr,
    r#gen: &mut FuncGen,
    funcs: &HashMap<String, (u32, bool)>,
) -> bool {
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
                    // extend i32 -> i64 so expressions consistently produce i64 values
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
                // Not equal: emit Eq then Eqz
                BinOp::NotEq => {
                    r#gen.instructions.push(IrInstruction::I64Eq);
                    // I64Eq produces an i32; use I32Eqz to invert that i32
                    r#gen.instructions.push(IrInstruction::I32Eqz);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                // <=  -> !(a > b)  =>  I64GtS ; I64Eqz
                BinOp::Le => {
                    r#gen.instructions.push(IrInstruction::I64GtS);
                    // I64GtS produces i32; invert with I32Eqz
                    r#gen.instructions.push(IrInstruction::I32Eqz);
                    r#gen.instructions.push(IrInstruction::I64ExtendI32S);
                }
                // >=  -> !(a < b)  =>  I64LtS ; I64Eqz
                BinOp::Ge => {
                    r#gen.instructions.push(IrInstruction::I64LtS);
                    // I64LtS produces i32; invert with I32Eqz
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
