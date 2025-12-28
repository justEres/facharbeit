use std::{collections::HashMap};

use wasm_encoder::Instruction;

use crate::{ast::{BinOp, Expr}, codegen::module::FuncGen};

pub fn emit_expr(expr: &Expr, r#gen: &mut FuncGen, funcs: &HashMap<String, u32>) {
    match expr{
        Expr::Int(v) => {
            r#gen.body.instruction(&Instruction::I64Const(*v));
        }
        Expr::Local(name) => {
            let idx = r#gen.local_map[name];
            r#gen.body.instruction(&Instruction::LocalGet(idx));
        }

        Expr::Binary { op, left, right } => {
            emit_expr(left, r#gen, funcs);
            emit_expr(right, r#gen, funcs);


            let instr = match op {
                BinOp::Add => Instruction::I64Add,
                BinOp::Sub => Instruction::I64Sub,
                BinOp::Mul => Instruction::I64Mul,
                BinOp::Div => Instruction::I32DivS,
                BinOp::Eq  => Instruction::I64Eq,
                BinOp::Lt  => Instruction::I64LtS,
                BinOp::Gt  => Instruction::I64GtS,
            };

            r#gen.body.instruction(&instr);
        }
        Expr::Call { name, args } => {
            for arg in args {
                emit_expr(arg, r#gen, funcs);
            }
            let idx = funcs[name];
            r#gen.body.instruction(&Instruction::Call(idx));
        }

    }
}
