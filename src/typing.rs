use std::collections::{HashMap, HashSet};

use crate::ast::*;
use crate::host::{abi_type_to_ast_type, host_descriptor_to_ast};
use eres_abi::{HostFunction, TypeDescriptor};

#[derive(Debug)]
pub struct TypedProgram {
    pub function_infos: HashMap<String, TypedFunction>,
    pub structs: HashMap<String, StructDecl>,
    pub enums: HashMap<String, EnumDecl>,
}

#[derive(Debug)]
pub struct TypedFunction {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
    pub locals: Vec<(String, Type)>,
    pub local_map: HashMap<String, (u32, Type)>,
}

#[derive(Debug)]
pub enum TypeError {
    UnknownType {
        name: String,
    },
    DuplicateType {
        kind: String,
        name: String,
    },
    DuplicateFunction {
        name: String,
    },
    UnknownFunction {
        name: String,
    },
    UnknownVariable {
        name: String,
    },
    DuplicateVariable {
        name: String,
    },
    InvalidFunctionReturn {
        function: String,
        expected: Type,
        found: Type,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
        what: String,
    },
    NotBooleanCondition {
        what: String,
        found: Type,
    },
    UnknownStruct {
        name: String,
    },
    UnknownEnum {
        name: String,
    },
    UnknownEnumVariant {
        enum_name: String,
        variant: String,
    },
    NonExhaustiveMatch {
        enum_name: String,
    },
    MatchPatternError {
        enum_name: String,
        variant: String,
        message: String,
    },
    DuplicateVariantPattern {
        enum_name: String,
        variant: String,
    },
    InvalidRefOperand {
        found: Type,
        what: String,
    },
    CannotInferEmptyList {
        what: String,
    },
    InvalidMainFunction {
        what: String,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UnknownType { name } => {
                write!(f, "unknown type `{}`", name)
            }
            TypeError::DuplicateType { kind, name } => {
                write!(f, "duplicate {} declaration `{}`", kind, name)
            }
            TypeError::DuplicateFunction { name } => {
                write!(f, "duplicate function declaration `{}`", name)
            }
            TypeError::UnknownFunction { name } => {
                write!(f, "unknown function `{}`", name)
            }
            TypeError::UnknownVariable { name } => {
                write!(f, "unknown variable `{}`", name)
            }
            TypeError::DuplicateVariable { name } => {
                write!(f, "duplicate local declaration `{}`", name)
            }
            TypeError::InvalidFunctionReturn {
                function,
                expected,
                found,
            } => {
                write!(
                    f,
                    "function `{}` return type mismatch: expected {}, found {}",
                    function,
                    expected,
                    found
                )
            }
            TypeError::TypeMismatch {
                expected,
                found,
                what,
            } => {
                write!(f, "{}: expected {}, found {}", what, expected, found)
            }
            TypeError::NotBooleanCondition { what, found } => {
                write!(f, "{} must be Bool, found {}", what, found)
            }
            TypeError::UnknownStruct { name } => write!(f, "unknown struct `{}`", name),
            TypeError::UnknownEnum { name } => write!(f, "unknown enum `{}`", name),
            TypeError::UnknownEnumVariant {
                enum_name,
                variant,
            } => {
                write!(
                    f,
                    "unknown variant `{}` in enum `{}`",
                    variant, enum_name
                )
            }
            TypeError::NonExhaustiveMatch { enum_name } => {
                write!(f, "non-exhaustive match on enum `{}`", enum_name)
            }
            TypeError::MatchPatternError {
                enum_name,
                variant,
                message,
            } => {
                write!(
                    f,
                    "invalid pattern for {}::{}: {}",
                    enum_name,
                    variant,
                    message
                )
            }
            TypeError::DuplicateVariantPattern {
                enum_name,
                variant,
            } => {
                write!(f, "match on {} has duplicated pattern {}", enum_name, variant)
            }
            TypeError::InvalidRefOperand { found, what } => {
                write!(f, "{}: expected reference, found {}", what, found)
            }
            TypeError::CannotInferEmptyList { what } => {
                write!(f, "{}", what)
            }
            TypeError::InvalidMainFunction { what } => {
                write!(f, "invalid `main` signature: {}", what)
            }
        }
    }
}

impl std::error::Error for TypeError {}

pub fn check_program(program: &Program) -> Result<TypedProgram, TypeError> {
    check_program_with_hosts(program, &[])
}

pub fn check_program_with_hosts(
    program: &Program,
    host_functions: &[HostFunction],
) -> Result<TypedProgram, TypeError> {
    let mut structs = HashMap::new();
    let mut enums = HashMap::new();
    let mut function_signatures = HashMap::new();

    for host in host_functions {
        register_host_descriptors(&mut structs, &mut enums, &host.descriptors)?;
        function_signatures.insert(
            host.name.to_string(),
            FnSignature {
                params: host
                    .params
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| Param {
                        name: format!("arg{}", idx + 1),
                        ty: abi_type_to_ast_type(ty),
                    })
                    .collect(),
                return_type: abi_type_to_ast_type(&host.result),
            },
        );
    }

    for item in &program.items {
        match item {
            TopLevelDecl::Use(_) => {}
            TopLevelDecl::Struct(def) => {
                if structs.contains_key(&def.name) {
                    return Err(TypeError::DuplicateType {
                        kind: "struct".to_string(),
                        name: def.name.clone(),
                    });
                }
                structs.insert(def.name.clone(), def.clone());
            }
            TopLevelDecl::Enum(def) => {
                if enums.contains_key(&def.name) {
                    return Err(TypeError::DuplicateType {
                        kind: "enum".to_string(),
                        name: def.name.clone(),
                    });
                }
                enums.insert(def.name.clone(), def.clone());
            }
            TopLevelDecl::Function(func) => {
                if function_signatures.contains_key(&func.name) {
                    return Err(TypeError::DuplicateFunction {
                        name: func.name.clone(),
                    });
                }
                function_signatures.insert(
                    func.name.clone(),
                    FnSignature {
                        params: func.params.clone(),
                        return_type: func.return_type.clone(),
                    },
                );
            }
        }
    }

    if let Some(main_sig) = function_signatures.get("main") {
        for (idx, p) in main_sig.params.iter().enumerate() {
            if p.ty != Type::Int {
                return Err(TypeError::InvalidMainFunction {
                    what: format!(
                        "main parameter {} `{}` must be Int, found {}",
                        idx + 1,
                        p.name,
                        p.ty
                    ),
                });
            }
        }
    }

    let mut function_infos = HashMap::new();

    for item in &program.items {
        if let TopLevelDecl::Function(func) = item {
            let info = infer_function(&function_signatures, &structs, &enums, func)?;
            function_infos.insert(func.name.clone(), info);
        }
    }

    Ok(TypedProgram {
        function_infos,
        structs,
        enums,
    })
}

#[derive(Clone)]
struct FnSignature {
    params: Vec<Param>,
    return_type: Type,
}

fn infer_function(
    function_signatures: &HashMap<String, FnSignature>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    func: &FunctionDecl,
) -> Result<TypedFunction, TypeError> {
    let mut local_map = HashMap::new();
    let mut locals = Vec::new();

    validate_named_type(structs, enums, &func.return_type)?;

    for (idx, param) in func.params.iter().enumerate() {
        validate_named_type(structs, enums, &param.ty)?;
        if local_map
            .insert(param.name.clone(), (idx as u32, param.ty.clone()))
            .is_some()
        {
            return Err(TypeError::DuplicateVariable {
                name: param.name.clone(),
            });
        }
    }

    for stmt in &func.body {
        infer_stmt(
            stmt,
            &mut local_map,
            &mut locals,
            structs,
            enums,
            function_signatures,
            &func.return_type,
        )?;
    }

    Ok(TypedFunction {
        name: func.name.clone(),
        params: func.params.clone(),
        return_type: func.return_type.clone(),
        locals,
        local_map,
    })
}

fn validate_named_type(
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    ty: &Type,
) -> Result<(), TypeError> {
    match ty {
        Type::Named(name) => {
            if structs.contains_key(name) || enums.contains_key(name) {
                Ok(())
            } else {
                Err(TypeError::UnknownType { name: name.clone() })
            }
        }
        Type::String => Ok(()),
        Type::Ref(inner) => validate_named_type(structs, enums, inner),
        Type::List(inner) => validate_named_type(structs, enums, inner),
        Type::Tuple(elements) => {
            for elem in elements {
                validate_named_type(structs, enums, elem)?;
            }
            Ok(())
        }
        Type::Function(args, ret) => {
            for arg in args {
                validate_named_type(structs, enums, arg)?;
            }
            validate_named_type(structs, enums, ret)
        }
        _ => Ok(()),
    }
}

fn infer_stmt(
    stmt: &Stmt,
    locals: &mut HashMap<String, (u32, Type)>,
    locals_decl: &mut Vec<(String, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
    expected_return: &Type,
) -> Result<(), TypeError> {
    match stmt {
        Stmt::Let { name, ty, value } => {
            if locals.contains_key(name) {
                return Err(TypeError::DuplicateVariable {
                    name: name.clone(),
                });
            }

            let value_type = match ty {
                Some(annotated) => infer_expr_with_expected(
                    value,
                    Some(annotated),
                    locals,
                    structs,
                    enums,
                    functions,
                )?,
                None => infer_expr(value, locals, structs, enums, functions)?,
            };
            let bound_type = match ty {
                Some(annotated) => {
                    validate_named_type(structs, enums, annotated)?;
                    if !is_assignable(annotated, &value_type) {
                        return Err(TypeError::TypeMismatch {
                            expected: annotated.clone(),
                            found: value_type,
                            what: format!("`{}` variable declaration", name),
                        });
                    }
                    annotated.clone()
                }
                None => value_type,
            };

            let index = locals.len() as u32;
            locals.insert(name.clone(), (index, bound_type));
            let bound = locals
                .get(name)
                .map(|(_, ty)| ty.clone())
                .ok_or_else(|| TypeError::UnknownVariable {
                    name: name.clone(),
                })?;
            locals_decl.push((name.clone(), bound));
            Ok(())
        }
        Stmt::Return(expr_opt) => match expr_opt {
            Some(expr) => {
                if expected_return == &Type::Unit {
                    return Err(TypeError::InvalidFunctionReturn {
                        function: "<return>".to_string(),
                        expected: Type::Unit,
                        found: infer_expr_with_expected(
                            expr,
                            Some(expected_return),
                            locals,
                            structs,
                            enums,
                            functions,
                        )?,
                    });
                }

                let found = infer_expr_with_expected(
                    expr,
                    Some(expected_return),
                    locals,
                    structs,
                    enums,
                    functions,
                )?;
                if !is_assignable(expected_return, &found) {
                    return Err(TypeError::InvalidFunctionReturn {
                        function: "<return>".to_string(),
                        expected: expected_return.clone(),
                        found,
                    });
                }
                Ok(())
            }
            None => {
                if expected_return != &Type::Unit {
                    return Err(TypeError::InvalidFunctionReturn {
                        function: "<return>".to_string(),
                        expected: expected_return.clone(),
                        found: Type::Unit,
                    });
                }
                Ok(())
            }
        },
        Stmt::Expr(expr) => {
            let _ = infer_expr(expr, locals, structs, enums, functions)?;
            Ok(())
        }
        Stmt::If {
            cond,
            then_block,
            else_block,
        } => {
            let cond_ty = infer_expr_with_expected(
                cond,
                Some(&Type::Bool),
                locals,
                structs,
                enums,
                functions,
            )?;
            if cond_ty != Type::Bool {
                return Err(TypeError::NotBooleanCondition {
                    what: "if condition".to_string(),
                    found: cond_ty,
                });
            }
            for stmt in then_block {
                infer_stmt(
                    stmt,
                    locals,
                    locals_decl,
                    structs,
                    enums,
                    functions,
                    expected_return,
                )?;
            }
            for stmt in else_block {
                infer_stmt(
                    stmt,
                    locals,
                    locals_decl,
                    structs,
                    enums,
                    functions,
                    expected_return,
                )?;
            }
            Ok(())
        }
        Stmt::While { cond, body } => {
            let cond_ty = infer_expr_with_expected(
                cond,
                Some(&Type::Bool),
                locals,
                structs,
                enums,
                functions,
            )?;
            if cond_ty != Type::Bool {
                return Err(TypeError::NotBooleanCondition {
                    what: "while condition".to_string(),
                    found: cond_ty,
                });
            }
            for stmt in body {
                infer_stmt(
                    stmt,
                    locals,
                    locals_decl,
                    structs,
                    enums,
                    functions,
                    expected_return,
                )?;
            }
            Ok(())
        }
    }
}

fn infer_expr(
    expr: &Expr,
    locals: &HashMap<String, (u32, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
) -> Result<Type, TypeError> {
    infer_expr_with_expected(expr, None, locals, structs, enums, functions)
}

fn infer_expr_with_expected(
    expr: &Expr,
    expected: Option<&Type>,
    locals: &HashMap<String, (u32, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
) -> Result<Type, TypeError> {
    match expr {
        Expr::Int(_) => Ok(Type::Int),
        Expr::Float(_) => Ok(Type::Float),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::String(_) => Ok(Type::String),
        Expr::Local(name) => locals
            .get(name)
            .map(|(_, ty)| ty.clone())
            .ok_or_else(|| TypeError::UnknownVariable { name: name.clone() }),
        Expr::Ref(inner) => {
            let inner_type = infer_expr_with_expected(inner, None, locals, structs, enums, functions)?;
            Ok(Type::Ref(Box::new(inner_type)))
        }
        Expr::Deref(inner) => {
            let inner_type = infer_expr_with_expected(inner, None, locals, structs, enums, functions)?;
            match inner_type {
                Type::Ref(inner) => Ok(*inner),
                _ => Err(TypeError::InvalidRefOperand {
                    found: inner_type,
                    what: "deref".to_string(),
                }),
            }
        }
        Expr::Binary { op, left, right } => infer_binary(op, left, right, locals, structs, enums, functions),
        Expr::Call { name, args } => {
            let sig = functions
                .get(name)
                .ok_or_else(|| TypeError::UnknownFunction { name: name.clone() })?;

            if sig.params.len() != args.len() {
                return Err(TypeError::TypeMismatch {
                    expected: Type::Named(format!("{} arguments", sig.params.len())),
                    found: Type::Named(format!("{} arguments", args.len())),
                    what: format!("function `{}` argument count", name),
                });
            }

            for (arg, param) in args.iter().zip(sig.params.iter()) {
                let arg_type = infer_expr_with_expected(arg, Some(&param.ty), locals, structs, enums, functions)?;
                if !is_assignable(&param.ty, &arg_type) {
                    return Err(TypeError::TypeMismatch {
                        expected: param.ty.clone(),
                        found: arg_type,
                        what: format!("argument for `{}`", name),
                    });
                }
            }
            Ok(sig.return_type.clone())
        }
        Expr::MethodCall {
            receiver,
            name,
            args,
        } => {
            let receiver_ty = infer_expr_with_expected(
                receiver,
                None,
                locals,
                structs,
                enums,
                functions,
            )?;

            match (name.as_str(), receiver_ty.clone()) {
                ("len", Type::List(_)) => {
                    if !args.is_empty() {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("0 arguments".to_string()),
                            found: Type::Named(format!("{} arguments", args.len())),
                            what: "method `len` call".to_string(),
                        });
                    }
                    Ok(Type::Int)
                }
                ("get", Type::List(inner)) => {
                    if args.len() != 1 {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("1 argument".to_string()),
                            found: Type::Named(format!("{} arguments", args.len())),
                            what: "method `get` call".to_string(),
                        });
                    }
                    let index = infer_expr_with_expected(
                        &args[0],
                        Some(&Type::Int),
                        locals,
                        structs,
                        enums,
                        functions,
                    )?;
                    if index != Type::Int {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Int,
                            found: index,
                            what: "method `get` index".to_string(),
                        });
                    }
                    Ok(*inner)
                }
                ("push", Type::List(inner)) => {
                    if args.len() != 1 {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("1 argument".to_string()),
                            found: Type::Named(format!("{} arguments", args.len())),
                            what: "method `push` call".to_string(),
                        });
                    }
                    let arg_ty = infer_expr_with_expected(
                        &args[0],
                        Some(&inner),
                        locals,
                        structs,
                        enums,
                        functions,
                    )?;
                    if !is_assignable(&inner, &arg_ty) {
                        return Err(TypeError::TypeMismatch {
                            expected: *inner.clone(),
                            found: arg_ty,
                            what: format!("method `push` argument for `List<{}>`", inner),
                        });
                    }
                    Ok(Type::Unit)
                }
                ("pop", Type::List(inner)) => {
                    if !args.is_empty() {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("0 arguments".to_string()),
                            found: Type::Named(format!("{} arguments", args.len())),
                            what: "method `pop` call".to_string(),
                        });
                    }
                    Ok(*inner)
                }
                (name, Type::List(_)) => Err(TypeError::MatchPatternError {
                    enum_name: "list".to_string(),
                    variant: name.to_string(),
                    message: "unknown list method; supported: len, get, push, pop".to_string(),
                }),
                (_, other) => Err(TypeError::TypeMismatch {
                    expected: Type::Named("list receiver".to_string()),
                    found: other,
                    what: "method call receiver".to_string(),
                }),
            }
        }
        Expr::ListLiteral(elems) => {
            if elems.is_empty() {
                match expected {
                    Some(Type::List(inner)) => Ok(Type::List(inner.clone())),
                    Some(other) => Err(TypeError::TypeMismatch {
                        expected: other.clone(),
                        found: Type::List(Box::new(Type::Unit)),
                        what: "empty list literal".to_string(),
                    }),
                    None => Err(TypeError::CannotInferEmptyList {
                        what: "cannot infer type of empty list literal without expected type annotation".to_string(),
                    }),
                }
            } else {
                if let Some(Type::List(inner)) = expected {
                    for elem in elems {
                        let elem_ty = infer_expr_with_expected(
                            elem,
                            Some(inner),
                            locals,
                            structs,
                            enums,
                            functions,
                        )?;
                        if !is_assignable(inner, &elem_ty) {
                            return Err(TypeError::TypeMismatch {
                                expected: *inner.clone(),
                                found: elem_ty,
                                what: "list element".to_string(),
                            });
                        }
                    }
                    return Ok(Type::List(inner.clone()));
                }

                let mut it = elems.iter();
                let first = infer_expr_with_expected(it.next().unwrap(), None, locals, structs, enums, functions)?;
                for (idx, elem) in it.enumerate() {
                    let elem_ty = infer_expr_with_expected(elem, None, locals, structs, enums, functions)?;
                    if !is_assignable(&first, &elem_ty) {
                        return Err(TypeError::TypeMismatch {
                            expected: first.clone(),
                            found: elem_ty,
                            what: format!("list element at index {}", idx + 1),
                        });
                    }
                }
                Ok(Type::List(Box::new(first)))
            }
        }
        Expr::TupleLiteral(fields) => {
            if let Some(Type::Tuple(expected_fields)) = expected {
                if fields.len() != expected_fields.len() {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Tuple(expected_fields.clone()),
                        found: Type::Named(format!("{} fields", fields.len())),
                        what: "tuple arity".to_string(),
                    });
                }
                for (idx, expr) in fields.iter().enumerate() {
                    let elem_ty = infer_expr_with_expected(
                        expr,
                        Some(&expected_fields[idx]),
                        locals,
                        structs,
                        enums,
                        functions,
                    )?;
                    if !is_assignable(&expected_fields[idx], &elem_ty) {
                        return Err(TypeError::TypeMismatch {
                            expected: expected_fields[idx].clone(),
                            found: elem_ty,
                            what: format!("tuple element {}", idx),
                        });
                    }
                }
                Ok(Type::Tuple(expected_fields.clone()))
            } else {
                let mut inferred = Vec::new();
                for expr in fields {
                    inferred.push(infer_expr_with_expected(
                        expr,
                        None,
                        locals,
                        structs,
                        enums,
                        functions,
                    )?);
                }
                Ok(Type::Tuple(inferred))
            }
        }
        Expr::Index { base, index } => {
            let base_ty = infer_expr_with_expected(base, None, locals, structs, enums, functions)?;
            let index_ty = infer_expr_with_expected(index, None, locals, structs, enums, functions)?;
            if index_ty != Type::Int {
                return Err(TypeError::TypeMismatch {
                    expected: Type::Int,
                    found: index_ty,
                    what: "index expression".to_string(),
                });
            }
            match base_ty {
                Type::List(inner) => Ok(*inner),
                Type::Tuple(fields) => match index.as_ref() {
                    Expr::Int(i) => {
                        if *i < 0 {
                            return Err(TypeError::TypeMismatch {
                                expected: Type::Named("non-negative index".to_string()),
                                found: Type::Int,
                                what: "tuple index".to_string(),
                            });
                        }
                        let idx = *i as usize;
                        fields
                            .get(idx)
                            .cloned()
                            .ok_or_else(|| TypeError::TypeMismatch {
                                expected: Type::Named(format!("tuple index < {}", fields.len())),
                                found: Type::Int,
                                what: format!("tuple index {}", idx),
                            })
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Int,
                        found: Type::Named("non-constant tuple index".to_string()),
                        what: "tuple index".to_string(),
                    }),
                },
                _ => Err(TypeError::TypeMismatch {
                    expected: Type::Named("List or Tuple".to_string()),
                    found: base_ty,
                    what: "indexing".to_string(),
                }),
            }
        }
        Expr::StructInit { name, fields } => {
            let def = structs
                .get(name)
                .ok_or_else(|| TypeError::UnknownStruct { name: name.clone() })?;

            let mut defined: HashMap<String, &Type> = HashMap::new();
            for (fname, fty) in &def.fields {
                defined.insert(fname.clone(), fty);
            }

            let mut seen = HashSet::new();
            for (fname, fexpr) in fields {
                let expected = match defined.get(fname) {
                    Some(ty) => ty,
                    None => {
                        return Err(TypeError::MatchPatternError {
                            enum_name: "struct".to_string(),
                            variant: fname.clone(),
                            message: "unknown field".to_string(),
                        });
                    }
                };

                if !seen.insert(fname.clone()) {
                    return Err(TypeError::MatchPatternError {
                        enum_name: name.clone(),
                        variant: fname.clone(),
                        message: "duplicate field".to_string(),
                    });
                }

                let found = infer_expr_with_expected(fexpr, Some(expected), locals, structs, enums, functions)?;
                if !is_assignable(expected, &found) {
                    return Err(TypeError::TypeMismatch {
                        expected: (*expected).clone(),
                        found,
                        what: format!("field `{}` in struct `{}`", fname, name),
                    });
                }
            }

            if fields.len() != def.fields.len() {
                return Err(TypeError::TypeMismatch {
                    expected: Type::Named(format!("{} fields", def.fields.len())),
                    found: Type::Named(format!("{} fields", fields.len())),
                    what: format!("struct `{}` construction", name),
                });
            }

            Ok(Type::Named(name.clone()))
        }
        Expr::EnumInit {
            enum_name,
            variant,
            payload,
        } => {
            let def = enums
                .get(enum_name)
                .ok_or_else(|| TypeError::UnknownEnum { name: enum_name.clone() })?;

            let variant_info = def
                .variants
                .iter()
                .find(|v| v.name() == *variant)
                .ok_or_else(|| TypeError::UnknownEnumVariant {
                    enum_name: enum_name.clone(),
                    variant: variant.clone(),
                })?;

            match variant_info {
                EnumVariant::Unit(_) => {
                    if !payload.is_empty() {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("0 payload elements".to_string()),
                            found: Type::Named(format!("{} payload elements", payload.len())),
                            what: format!("payload for {}::{}", enum_name, variant),
                        });
                    }
                }
                EnumVariant::Tuple(_, payload_type) => {
                    if payload.len() != 1 {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named("1 payload element".to_string()),
                            found: Type::Named(format!("{} payload elements", payload.len())),
                            what: format!("payload for {}::{}", enum_name, variant),
                        });
                    }
                    let found =
                        infer_expr_with_expected(&payload[0], Some(payload_type), locals, structs, enums, functions)?;
                    if !is_assignable(payload_type, &found) {
                        return Err(TypeError::TypeMismatch {
                            expected: payload_type.clone(),
                            found,
                            what: format!("payload for {}::{}", enum_name, variant),
                        });
                    }
                }
                EnumVariant::Struct(_, fields) => {
                    if payload.len() != fields.len() {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Named(format!("{} payload fields", fields.len())),
                            found: Type::Named(format!("{} payload fields", payload.len())),
                            what: format!("payload for {}::{}", enum_name, variant),
                        });
                    }
                    for (idx, expected_field) in fields.iter().enumerate() {
                        let found = infer_expr_with_expected(
                            &payload[idx],
                            Some(&expected_field.1),
                            locals,
                            structs,
                            enums,
                            functions,
                        )?;
                        if !is_assignable(&expected_field.1, &found) {
                            return Err(TypeError::TypeMismatch {
                                expected: expected_field.1.clone(),
                                found,
                                what: format!(
                                    "payload index {} for {}::{}",
                                    idx, enum_name, variant
                                ),
                            });
                        }
                    }
                }
            }

            Ok(Type::Named(enum_name.clone()))
        }
        Expr::Match { subject, arms } => {
            let subject_type = infer_expr_with_expected(subject, None, locals, structs, enums, functions)?;
            let enum_name = match subject_type {
                Type::Named(name) => {
                    if !enums.contains_key(&name) {
                        return Err(TypeError::UnknownEnum { name });
                    }
                    name
                }
                _ => {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Named("enum type".to_string()),
                        found: subject_type,
                        what: "match subject".to_string(),
                    })
                }
            };

            let def = enums
                .get(&enum_name)
                .ok_or_else(|| TypeError::UnknownEnum { name: enum_name.clone() })?;

            let mut used = HashSet::new();
            let mut arm_types: Option<Type> = None;

            for arm in arms {
                let variant_ty = infer_match_variant(
                    arm,
                    &enum_name,
                    def,
                    locals,
                    structs,
                    enums,
                    functions,
                )?;

                let variant_name = arm.pattern.variant_name();
                if !used.insert(variant_name.clone()) {
                    return Err(TypeError::DuplicateVariantPattern {
                        enum_name: enum_name.clone(),
                        variant: variant_name,
                    });
                }

                match &arm_types {
                    Some(prev) if prev != &variant_ty => {
                        return Err(TypeError::TypeMismatch {
                            expected: prev.clone(),
                            found: variant_ty,
                            what: "match arm body type".to_string(),
                        });
                    }
                    None => arm_types = Some(variant_ty),
                    _ => {}
                }
            }

            if used.len() != def.variants.len() {
                return Err(TypeError::NonExhaustiveMatch {
                    enum_name: enum_name.clone(),
                });
            }

            Ok(arm_types.unwrap_or(Type::Unit))
        }
    }
}

fn infer_binary(
    op: &BinOp,
    left: &Expr,
    right: &Expr,
    locals: &HashMap<String, (u32, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
) -> Result<Type, TypeError> {
    let left_ty = infer_expr_with_expected(left, None, locals, structs, enums, functions)?;
    let right_ty = infer_expr_with_expected(right, None, locals, structs, enums, functions)?;
    if !types_match(&left_ty, &right_ty) {
        return Err(TypeError::TypeMismatch {
            expected: left_ty,
            found: right_ty,
            what: format!("binary operator {:?}", op),
        });
    }

    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => match left_ty {
            Type::Int => Ok(Type::Int),
            Type::Float => Ok(Type::Float),
            _ => Err(TypeError::TypeMismatch {
                expected: Type::Named("Int or Float".to_string()),
                found: left_ty,
                what: "arithmetic expression".to_string(),
            }),
        },
        BinOp::Eq | BinOp::NotEq => match left_ty {
            Type::Int | Type::Float | Type::Bool | Type::String => Ok(Type::Bool),
            _ => Err(TypeError::TypeMismatch {
                expected: Type::Named("Int, Float, Bool, or String".to_string()),
                found: left_ty,
                what: "equality expression".to_string(),
            }),
        },
        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge => match left_ty {
            Type::Int | Type::Float => Ok(Type::Bool),
            _ => Err(TypeError::TypeMismatch {
                expected: Type::Named("Int or Float".to_string()),
                found: left_ty,
                what: "ordering expression".to_string(),
            }),
        },
    }
}

fn infer_match_variant(
    arm: &MatchArm,
    enum_name: &String,
    def: &EnumDecl,
    locals: &HashMap<String, (u32, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
) -> Result<Type, TypeError> {
    let variant_name = arm.pattern.variant_name();
    let variant = def
        .variants
        .iter()
        .find(|v| v.name() == variant_name)
        .ok_or_else(|| TypeError::UnknownEnumVariant {
            enum_name: enum_name.clone(),
            variant: variant_name,
        })?;

    let mut scope = locals.clone();
    match arm.pattern.clone() {
        Pattern::UnitVariant(_) => {
            arm_body_type(&arm.body, &scope, structs, enums, functions)
        }
        Pattern::TupleVariant(_, vars) => match variant {
            EnumVariant::Tuple(_, payload_ty) => {
                if vars.len() != 1 {
                    return Err(TypeError::MatchPatternError {
                        enum_name: enum_name.clone(),
                        variant: arm.pattern.variant_name(),
                        message: "tuple variant expects exactly one binding".to_string(),
                    });
                }
                let var_name = vars[0].clone();
                if !scope.insert(var_name.clone(), (scope.len() as u32, payload_ty.clone())).is_none() {
                    return Err(TypeError::DuplicateVariable {
                        name: var_name,
                    });
                }
                arm_body_type(&arm.body, &scope, structs, enums, functions)
            }
            _ => Err(TypeError::MatchPatternError {
                enum_name: enum_name.clone(),
                variant: arm.pattern.variant_name(),
                message: "not a tuple variant".to_string(),
            }),
        },
        Pattern::StructVariant(_, vars) => match variant {
            EnumVariant::Struct(_, fields) => {
                if vars.len() != fields.len() {
                    return Err(TypeError::MatchPatternError {
                        enum_name: enum_name.clone(),
                        variant: arm.pattern.variant_name(),
                        message: "struct variant binding count mismatch".to_string(),
                    });
                }

                let mut seen = HashSet::new();
                for field_name in vars {
                    if !seen.insert(field_name.clone()) {
                        return Err(TypeError::MatchPatternError {
                            enum_name: enum_name.clone(),
                            variant: arm.pattern.variant_name(),
                            message: "duplicate binding".to_string(),
                        });
                    }
                    if let Some((_, field_ty)) = fields.iter().find(|(name, _)| name == &field_name) {
                        if !scope
                            .insert(field_name.clone(), (scope.len() as u32, field_ty.clone()))
                            .is_none()
                        {
                            return Err(TypeError::DuplicateVariable {
                                name: field_name,
                            });
                        }
                    } else {
                        return Err(TypeError::MatchPatternError {
                            enum_name: enum_name.clone(),
                            variant: arm.pattern.variant_name(),
                            message: format!("unknown field `{}`", field_name),
                        });
                    }
                }
                arm_body_type(&arm.body, &scope, structs, enums, functions)
            }
            _ => Err(TypeError::MatchPatternError {
                enum_name: enum_name.clone(),
                variant: arm.pattern.variant_name(),
                message: "not a struct variant".to_string(),
            }),
        },
    }
}

fn arm_body_type(
    body: &Expr,
    locals: &HashMap<String, (u32, Type)>,
    structs: &HashMap<String, StructDecl>,
    enums: &HashMap<String, EnumDecl>,
    functions: &HashMap<String, FnSignature>,
) -> Result<Type, TypeError> {
    infer_expr(body, locals, structs, enums, functions)
}

fn is_assignable(expected: &Type, provided: &Type) -> bool {
    match (expected, provided) {
        (expected, provided) if types_match(expected, provided) => true,
        (Type::List(exp_elem), Type::List(prov_elem)) => {
            is_assignable(exp_elem, prov_elem)
        }
        (Type::Tuple(exp_fields), Type::Tuple(prov_fields)) => {
            if exp_fields.len() != prov_fields.len() {
                return false;
            }
            for (exp_ty, prov_ty) in exp_fields.iter().zip(prov_fields.iter()) {
                if !is_assignable(exp_ty, prov_ty) {
                    return false;
                }
            }
            true
        }
        (_, Type::Ref(inner)) => is_assignable(expected, inner),
        _ => false,
    }
}

fn register_host_descriptors(
    structs: &mut HashMap<String, StructDecl>,
    enums: &mut HashMap<String, EnumDecl>,
    descriptors: &[TypeDescriptor],
) -> Result<(), TypeError> {
    for descriptor in descriptors {
        match host_descriptor_to_ast(descriptor) {
            Some(TopLevelDecl::Struct(def)) => {
                if let Some(existing) = structs.get(&def.name) {
                    if existing.fields != def.fields {
                        return Err(TypeError::DuplicateType {
                            kind: "struct".to_string(),
                            name: def.name.clone(),
                        });
                    }
                } else {
                    structs.insert(def.name.clone(), def);
                }
            }
            Some(TopLevelDecl::Enum(def)) => {
                if let Some(existing) = enums.get(&def.name) {
                    if existing.variants.len() != def.variants.len() {
                        return Err(TypeError::DuplicateType {
                            kind: "enum".to_string(),
                            name: def.name.clone(),
                        });
                    }
                } else {
                    enums.insert(def.name.clone(), def);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn types_match(a: &Type, b: &Type) -> bool {
    a == b
}

impl Pattern {
    fn variant_name(&self) -> String {
        match self {
            Pattern::UnitVariant(v) => v.clone(),
            Pattern::TupleVariant(v, _) => v.clone(),
            Pattern::StructVariant(v, _) => v.clone(),
        }
    }
}

impl EnumVariant {
    fn name(&self) -> String {
        match self {
            EnumVariant::Unit(name) => name.clone(),
            EnumVariant::Tuple(name, _) => name.clone(),
            EnumVariant::Struct(name, _) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex_file;
    use crate::parser::Parser;

    fn type_check(src: &str) -> Result<TypedProgram, TypeError> {
        let tokens = lex_file(src).unwrap();
        let mut parser = Parser::new(&tokens);
        let program = parser.parse_program().unwrap();
        check_program_with_hosts(&program, &[])
    }

    #[test]
    fn list_literal_rejects_heterogeneous_elements() {
        let src = "fn main() -> Int { let xs = [1, true, 2]; return 0; }";
        assert!(type_check(src).is_err());
    }

    #[test]
    fn tuple_literal_allows_mixed_types_and_tuple_assignment() {
        let src = "fn main() -> Int { let t = (1, 2.0); let _: Int = 0; return 0; }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn tuple_annotation_enforces_arity_and_field_types() {
        let src = "fn main() -> Int { let t = (1, true); let u: (Int, Float) = t; return 0; }";
        assert!(type_check(src).is_err());
    }

    #[test]
    fn tuple_return_type_accepted() {
        let src = "fn pair() -> (Int, Float) { return (1, 2.0); } fn main() -> (Int, Float) { return pair(); }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn list_return_and_index_usage() {
        let src = "fn first(x: List<Int>) -> Int { return x[0]; } fn main() -> Int { return first([1, 2, 3]); }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn tuple_dot_index_is_supported() {
        let src = "fn head(x: (Int, Float)) -> Int { return x.0; }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn list_method_call_typechecking() {
        let src = "fn f(x: List<Int>) -> Int { let _: List<Int> = x; x.push(1); return x.len(); }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn list_push_argument_type_is_checked() {
        let src = "fn f(x: List<Int>) -> Int { let _: List<Int> = x; x.push(true); return x.len(); }";
        assert!(type_check(src).is_err());
    }

    #[test]
    fn empty_list_requires_context() {
        let src = "fn main() -> Int { let xs = []; return 0; }";
        assert!(type_check(src).is_err());
    }

    #[test]
    fn string_equality_typechecks() {
        let src = "fn main() -> Bool { return \"a\" == \"b\"; }";
        assert!(type_check(src).is_ok());
    }

    #[test]
    fn string_arithmetic_is_rejected() {
        let src = "fn main() -> String { return \"a\" + \"b\"; }";
        assert!(type_check(src).is_err());
    }
}
