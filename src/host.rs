use crate::ast::{EnumDecl, EnumVariant, StructDecl, TopLevelDecl, Type};
use eres_abi::{
    AbiType, EnumVariantDescriptor, HostFunction, NamedTypeKind, TypeDescriptor, TypeDescriptorKind,
};

pub fn default_host_functions() -> Vec<HostFunction> {
    eres_std::standard_library()
}

pub fn abi_type_to_ast_type(ty: &AbiType) -> Type {
    match ty {
        AbiType::Int => Type::Int,
        AbiType::Float => Type::Float,
        AbiType::Bool => Type::Bool,
        AbiType::String => Type::String,
        AbiType::List(inner) => Type::List(Box::new(abi_type_to_ast_type(inner))),
        AbiType::Tuple(elements) => Type::Tuple(elements.iter().map(abi_type_to_ast_type).collect()),
        AbiType::Named(named) => Type::Named(named.name.clone()),
        AbiType::Unit => Type::Unit,
    }
}

pub fn host_descriptor_to_ast(descriptor: &TypeDescriptor) -> Option<TopLevelDecl> {
    match (&descriptor.named.kind, &descriptor.kind) {
        (NamedTypeKind::Struct, TypeDescriptorKind::Struct { fields }) => {
            Some(TopLevelDecl::Struct(StructDecl {
                name: descriptor.named.name.clone(),
                fields: fields
                    .iter()
                    .map(|field| (field.name.clone(), abi_type_to_ast_type(&field.ty)))
                    .collect(),
            }))
        }
        (NamedTypeKind::Enum, TypeDescriptorKind::Enum { variants }) => {
            Some(TopLevelDecl::Enum(EnumDecl {
                name: descriptor.named.name.clone(),
                variants: variants
                    .iter()
                    .map(|variant| match variant {
                        EnumVariantDescriptor::Unit { name } => EnumVariant::Unit(name.clone()),
                        EnumVariantDescriptor::Tuple { name, ty } => {
                            EnumVariant::Tuple(name.clone(), abi_type_to_ast_type(ty))
                        }
                        EnumVariantDescriptor::Struct { name, fields } => EnumVariant::Struct(
                            name.clone(),
                            fields
                                .iter()
                                .map(|field| (field.name.clone(), abi_type_to_ast_type(&field.ty)))
                                .collect(),
                        ),
                    })
                    .collect(),
            }))
        }
        _ => None,
    }
}
