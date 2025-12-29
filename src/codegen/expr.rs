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

            let instr = match op {
                BinOp::Add => IrInstruction::I64Add,
                BinOp::Sub => IrInstruction::I64Sub,
                BinOp::Mul => IrInstruction::I64Mul,
                BinOp::Div => IrInstruction::I64DivS,
                BinOp::Eq => IrInstruction::I64Eq,
                BinOp::Lt => IrInstruction::I64LtS,
                BinOp::Gt => IrInstruction::I64GtS,
            };

            r#gen.instructions.push(instr);
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
