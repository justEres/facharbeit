use std::collections::HashMap;

use crate::ast::{BinOp, Expr, Type};
use crate::codegen::ir::IrInstruction;
use crate::codegen::module::{CodegenError, FuncGen, FunctionSig};

/// Emits instructions for an expression and returns whether it leaves a stack value.
pub fn emit_expr(
    expr: &Expr,
    cg: &mut FuncGen,
    funcs: &HashMap<String, FunctionSig>,
) -> Result<bool, CodegenError> {
    match expr {
        Expr::Int(v) => {
            cg.instructions.push(IrInstruction::I64Const(*v));
            Ok(true)
        }
        Expr::Float(v) => {
            cg.instructions.push(IrInstruction::F64Const(*v));
            Ok(true)
        }
        Expr::Bool(v) => {
            let bit = if *v { 1 } else { 0 };
            cg.instructions.push(IrInstruction::I32Const(bit));
            Ok(true)
        }
        Expr::ListLiteral(_) => {
            Err(CodegenError::UnsupportedType("list literal lowering".to_string()))
        }
        Expr::TupleLiteral(_) => {
            Err(CodegenError::UnsupportedType("tuple literal lowering".to_string()))
        }
        Expr::Index { .. } => {
            Err(CodegenError::UnsupportedType("index expression lowering".to_string()))
        }
        Expr::MethodCall { .. } => {
            Err(CodegenError::UnsupportedType(
                "method call lowering".to_string(),
            ))
        }
        Expr::Local(name) => {
            let idx = cg
                .local_map
                .get(name)
                .ok_or_else(|| CodegenError::UnknownLocal { name: name.clone() })?;
            cg.instructions.push(IrInstruction::LocalGet(idx.0));
            Ok(true)
        }
        Expr::Ref(_inner) => {
            Err(CodegenError::UnsupportedType("&expr lowering".to_string()))
        }
        Expr::Deref(_inner) => {
            Err(CodegenError::UnsupportedType("*expr lowering".to_string()))
        }
            Expr::Binary { op, left, right } => {
            let left_ty = infer_expr_type(left, cg, funcs)?;
            let right_ty = infer_expr_type(right, cg, funcs)?;
            if left_ty != right_ty {
                return Err(CodegenError::UnsupportedType(format!(
                    "binary type mismatch: expected {}, found {}",
                    left_ty, right_ty
                )));
            }

            let _ = emit_expr(left, cg, funcs)?;
            let _ = emit_expr(right, cg, funcs)?;

            match (op, &left_ty) {
                (BinOp::Add, Type::Int) => cg.instructions.push(IrInstruction::I64Add),
                (BinOp::Sub, Type::Int) => cg.instructions.push(IrInstruction::I64Sub),
                (BinOp::Mul, Type::Int) => cg.instructions.push(IrInstruction::I64Mul),
                (BinOp::Div, Type::Int) => cg.instructions.push(IrInstruction::I64DivS),
                (BinOp::Add, Type::Float) => cg.instructions.push(IrInstruction::F64Add),
                (BinOp::Sub, Type::Float) => cg.instructions.push(IrInstruction::F64Sub),
                (BinOp::Mul, Type::Float) => cg.instructions.push(IrInstruction::F64Mul),
                (BinOp::Div, Type::Float) => cg.instructions.push(IrInstruction::F64Div),
                (BinOp::Eq, Type::Int) => cg.instructions.push(IrInstruction::I64Eq),
                (BinOp::Eq, Type::Float) => cg.instructions.push(IrInstruction::F64Eq),
                (BinOp::Eq, Type::Bool) => cg.instructions.push(IrInstruction::I32Eq),
                (BinOp::NotEq, Type::Int) => {
                    cg.instructions.push(IrInstruction::I64Eq);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::NotEq, Type::Float) => {
                    cg.instructions.push(IrInstruction::F64Eq);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::NotEq, Type::Bool) => {
                    cg.instructions.push(IrInstruction::I32Eq);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::Lt, Type::Int) => cg.instructions.push(IrInstruction::I64LtS),
                (BinOp::Lt, Type::Float) => cg.instructions.push(IrInstruction::F64Lt),
                (BinOp::Gt, Type::Int) => cg.instructions.push(IrInstruction::I64GtS),
                (BinOp::Gt, Type::Float) => cg.instructions.push(IrInstruction::F64Gt),
                (BinOp::Le, Type::Int) => {
                    cg.instructions.push(IrInstruction::I64GtS);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::Le, Type::Float) => {
                    cg.instructions.push(IrInstruction::F64Gt);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::Ge, Type::Int) => {
                    cg.instructions.push(IrInstruction::I64LtS);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                (BinOp::Ge, Type::Float) => {
                    cg.instructions.push(IrInstruction::F64Lt);
                    cg.instructions.push(IrInstruction::I32Eqz);
                }
                _ => {
                    return Err(CodegenError::UnsupportedType(format!(
                        "unsupported operator {:?} for type {}",
                        op, left_ty
                    )));
                }
            }

            Ok(true)
        }
        Expr::Call { name, args } => {
            for arg in args {
                let _ = emit_expr(arg, cg, funcs)?;
            }
            let (idx, _params, ret) = funcs
                .get(name)
                .cloned()
                .ok_or_else(|| CodegenError::UnknownFunction { name: name.clone() })?;
            cg.instructions.push(IrInstruction::Call(idx));
            Ok(ret != Type::Unit)
        }
        Expr::StructInit { name: _, fields: _ } => Err(CodegenError::UnsupportedType(
            "struct initialization lowering".to_string(),
        )),
        Expr::EnumInit {
            enum_name: _,
            variant: _,
            payload: _,
        } => Err(CodegenError::UnsupportedType(
            "enum initialization lowering".to_string(),
        )),
        Expr::Match { .. } => Err(CodegenError::UnsupportedType("match lowering".to_string())),
    }
}

pub(crate) fn infer_expr_type(
    expr: &Expr,
    cg: &FuncGen,
    funcs: &HashMap<String, FunctionSig>,
) -> Result<Type, CodegenError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::ListLiteral(_) => Err(CodegenError::UnsupportedType(
            "list literal lowering".to_string(),
        )),
        Expr::TupleLiteral(_) => Err(CodegenError::UnsupportedType(
            "tuple literal lowering".to_string(),
        )),
        Expr::Index { .. } => Err(CodegenError::UnsupportedType(
            "index expression lowering".to_string(),
        )),
        Expr::MethodCall { .. } => Err(CodegenError::UnsupportedType(
            "method call lowering".to_string(),
        )),
        Expr::Local(name) => cg
            .local_map
            .get(name)
            .map(|(_, ty)| ty.clone())
            .ok_or_else(|| CodegenError::UnknownLocal { name: name.clone() }),
        Expr::Ref(_) => Err(CodegenError::UnsupportedType("&expr lowering".to_string())),
        Expr::Deref(inner) => match infer_expr_type(inner, cg, funcs)? {
            Type::Ref(inner) => Ok(*inner),
            _ => Err(CodegenError::UnsupportedType("*expr lowering".to_string())),
        },
            Expr::Binary { op, left, right } => {
                let left_ty = infer_expr_type(left, cg, funcs)?;
                let right_ty = infer_expr_type(right, cg, funcs)?;
                if left_ty != right_ty {
                    return Err(CodegenError::UnsupportedType(format!(
                        "binary type mismatch: {} vs {}",
                    left_ty, right_ty
                )));
            }
                match (op, left_ty.clone()) {
                    (
                        BinOp::Eq | BinOp::NotEq,
                        Type::Int | Type::Float | Type::Bool,
                    ) => Ok(Type::Bool),
                    (BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge, Type::Int | Type::Float) => {
                        Ok(Type::Bool)
                    }
                    (BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div, Type::Int | Type::Float) => {
                        Ok(left_ty)
                    }
                    _ => Err(CodegenError::UnsupportedType(format!(
                        "unsupported binary type {}",
                        left_ty
                    ))),
                }
            }
        Expr::Call { name, .. } => {
            let (_idx, _params, ret) = funcs
                .get(name)
                .cloned()
                .ok_or_else(|| CodegenError::UnknownFunction { name: name.clone() })?;
            Ok(ret)
        }
        Expr::StructInit { .. } => Err(CodegenError::UnsupportedType(
            "struct initialization lowering".to_string(),
        )),
        Expr::EnumInit { .. } => Err(CodegenError::UnsupportedType(
            "enum initialization lowering".to_string(),
        )),
        Expr::Match { .. } => Err(CodegenError::UnsupportedType("match lowering".to_string())),
    }
}
