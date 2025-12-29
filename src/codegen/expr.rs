use std::collections::HashMap;



use crate::{
    ast::{BinOp, Expr},
    codegen::module::FuncGen,
    codegen::ir::IrInstruction,
};

pub fn emit_expr(expr: &Expr, r#gen: &mut FuncGen, funcs: &HashMap<String, u32>) {
    match expr {
        Expr::Int(v) => {
            r#gen.instructions.push(IrInstruction::I64Const(*v));
        }
        Expr::Local(name) => {
            let idx = r#gen.local_map[name];
            r#gen.instructions.push(IrInstruction::LocalGet(idx));
        }

        Expr::Binary { op, left, right } => {
            emit_expr(left, r#gen, funcs);
            emit_expr(right, r#gen, funcs);

            match op {
                BinOp::Add => r#gen.instructions.push(IrInstruction::I64Add),
                BinOp::Sub => r#gen.instructions.push(IrInstruction::I64Sub),
                BinOp::Mul => r#gen.instructions.push(IrInstruction::I64Mul),
                BinOp::Div => r#gen.instructions.push(IrInstruction::I64DivS),
                BinOp::Eq => r#gen.instructions.push(IrInstruction::I64Eq),
                BinOp::Lt => r#gen.instructions.push(IrInstruction::I64LtS),
                BinOp::Gt => r#gen.instructions.push(IrInstruction::I64GtS),
                // Not equal: emit Eq then Eqz
                BinOp::NotEq => {
                    r#gen.instructions.push(IrInstruction::I64Eq);
                    r#gen.instructions.push(IrInstruction::I64Eqz);
                }
                // <=  -> !(a > b)  =>  I64GtS ; I64Eqz
                BinOp::Le => {
                    r#gen.instructions.push(IrInstruction::I64GtS);
                    r#gen.instructions.push(IrInstruction::I64Eqz);
                }
                // >=  -> !(a < b)  =>  I64LtS ; I64Eqz
                BinOp::Ge => {
                    r#gen.instructions.push(IrInstruction::I64LtS);
                    r#gen.instructions.push(IrInstruction::I64Eqz);
                }
            }
        }
        Expr::Call { name, args } => {
            for arg in args {
                emit_expr(arg, r#gen, funcs);
            }
            let idx = funcs[name];
            r#gen.instructions.push(IrInstruction::Call(idx));
        }
    }
}
