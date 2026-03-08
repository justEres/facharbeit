use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EresAbi)]
pub fn derive_eres_abi(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match &input.data {
        Data::Struct(data) => derive_struct(&input, data).into(),
        Data::Enum(data) => derive_enum(&input, data).into(),
        Data::Union(_) => syn::Error::new_spanned(&input, "EresAbi does not support unions")
            .to_compile_error()
            .into(),
    }
}

fn derive_struct(input: &DeriveInput, data: &DataStruct) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let field_idents = match &data.fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| field.ident.clone().expect("named field"))
            .collect::<Vec<_>>(),
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "EresAbi only supports structs with named fields",
            )
            .to_compile_error();
        }
    };
    let field_types = match &data.fields {
        Fields::Named(fields) => fields.named.iter().map(|field| &field.ty).collect::<Vec<_>>(),
        _ => unreachable!(),
    };
    let field_names = field_idents.iter().map(|ident| ident.to_string()).collect::<Vec<_>>();

    quote! {
        impl #ident {
            fn __eres_named_type() -> ::eres_abi::NamedTypeRef {
                ::eres_abi::NamedTypeRef::new(
                    ::std::any::type_name::<Self>(),
                    stringify!(#ident),
                    ::eres_abi::NamedTypeKind::Struct,
                )
            }
        }

        impl ::eres_abi::EresAbi for #ident {
            fn abi_type() -> ::eres_abi::AbiType {
                ::eres_abi::AbiType::Named(Self::__eres_named_type())
            }

            fn descriptors() -> Vec<::eres_abi::TypeDescriptor> {
                let mut registry = ::eres_abi::DescriptorRegistry::default();
                #(registry.extend(<#field_types as ::eres_abi::EresAbi>::descriptors());)*
                registry.insert(::eres_abi::TypeDescriptor {
                    named: Self::__eres_named_type(),
                    kind: ::eres_abi::TypeDescriptorKind::Struct {
                        fields: vec![
                            #(
                                ::eres_abi::FieldDescriptor {
                                    name: #field_names.to_string(),
                                    ty: <#field_types as ::eres_abi::EresAbi>::abi_type(),
                                }
                            ),*
                        ],
                    },
                });
                registry.finish()
            }
        }

        impl ::eres_abi::IntoRuntimeValue for #ident {
            fn into_runtime_value(self) -> Result<::eres_abi::RuntimeValue, String> {
                Ok(::eres_abi::RuntimeValue::Struct {
                    named: Self::__eres_named_type(),
                    fields: vec![
                        #(
                            (
                                #field_names.to_string(),
                                ::eres_abi::IntoRuntimeValue::into_runtime_value(self.#field_idents)?,
                            )
                        ),*
                    ],
                })
            }
        }

        impl ::eres_abi::FromRuntimeValue for #ident {
            fn from_runtime_value(value: &::eres_abi::RuntimeValue) -> Result<Self, String> {
                match value {
                    ::eres_abi::RuntimeValue::Struct { named, fields } => {
                        let expected = Self::__eres_named_type();
                        if named.type_id != expected.type_id {
                            return Err(format!(
                                "expected struct type {}, found {}",
                                expected.name,
                                named.name
                            ));
                        }
                        let mut map = ::std::collections::BTreeMap::new();
                        for (name, value) in fields {
                            map.insert(name.as_str(), value);
                        }
                        Ok(Self {
                            #(
                                #field_idents: <#field_types as ::eres_abi::FromRuntimeValue>::from_runtime_value(
                                    map.remove(#field_names)
                                        .ok_or_else(|| format!("missing field `{}`", #field_names))?
                                )?,
                            )*
                        })
                    }
                    other => Err(format!("expected struct runtime value, found {:?}", other)),
                }
            }
        }

        impl ::eres_abi::ComplexEresAbi for #ident {}
    }
}

fn derive_enum(input: &DeriveInput, data: &DataEnum) -> proc_macro2::TokenStream {
    let ident = &input.ident;
    let mut descriptor_variants = Vec::new();
    let mut into_arms = Vec::new();
    let mut from_arms = Vec::new();
    let mut nested_types = Vec::new();

    for variant in &data.variants {
        let vident = &variant.ident;
        let vname = vident.to_string();
        match &variant.fields {
            Fields::Unit => {
                descriptor_variants.push(quote! {
                    ::eres_abi::EnumVariantDescriptor::Unit { name: #vname.to_string() }
                });
                into_arms.push(quote! {
                    Self::#vident => Ok(::eres_abi::RuntimeValue::Enum {
                        named: Self::__eres_named_type(),
                        variant: #vname.to_string(),
                        payload: ::eres_abi::EnumPayload::Unit,
                    })
                });
                from_arms.push(quote! {
                    #vname => match payload {
                        ::eres_abi::EnumPayload::Unit => Ok(Self::#vident),
                        other => Err(format!("expected unit payload for {}, found {:?}", #vname, other)),
                    }
                });
            }
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return syn::Error::new_spanned(
                        &variant.ident,
                        "EresAbi only supports single-field tuple enum variants",
                    )
                    .to_compile_error();
                }
                let ty = &fields.unnamed.first().expect("tuple field").ty;
                nested_types.push(quote! { #ty });
                descriptor_variants.push(quote! {
                    ::eres_abi::EnumVariantDescriptor::Tuple {
                        name: #vname.to_string(),
                        ty: <#ty as ::eres_abi::EresAbi>::abi_type(),
                    }
                });
                into_arms.push(quote! {
                    Self::#vident(inner) => Ok(::eres_abi::RuntimeValue::Enum {
                        named: Self::__eres_named_type(),
                        variant: #vname.to_string(),
                        payload: ::eres_abi::EnumPayload::Tuple(Box::new(
                            ::eres_abi::IntoRuntimeValue::into_runtime_value(inner)?
                        )),
                    })
                });
                from_arms.push(quote! {
                    #vname => match payload {
                        ::eres_abi::EnumPayload::Tuple(inner) => Ok(Self::#vident(
                            <#ty as ::eres_abi::FromRuntimeValue>::from_runtime_value(inner.as_ref())?
                        )),
                        other => Err(format!("expected tuple payload for {}, found {:?}", #vname, other)),
                    }
                });
            }
            Fields::Named(fields) => {
                let field_idents = fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone().expect("named field"))
                    .collect::<Vec<_>>();
                let field_types = fields.named.iter().map(|field| &field.ty).collect::<Vec<_>>();
                let field_names = field_idents.iter().map(|field| field.to_string()).collect::<Vec<_>>();
                nested_types.extend(field_types.iter().map(|ty| quote! { #ty }));
                descriptor_variants.push(quote! {
                    ::eres_abi::EnumVariantDescriptor::Struct {
                        name: #vname.to_string(),
                        fields: vec![
                            #(
                                ::eres_abi::FieldDescriptor {
                                    name: #field_names.to_string(),
                                    ty: <#field_types as ::eres_abi::EresAbi>::abi_type(),
                                }
                            ),*
                        ],
                    }
                });
                into_arms.push(quote! {
                    Self::#vident { #(#field_idents),* } => Ok(::eres_abi::RuntimeValue::Enum {
                        named: Self::__eres_named_type(),
                        variant: #vname.to_string(),
                        payload: ::eres_abi::EnumPayload::Struct(vec![
                            #(
                                (
                                    #field_names.to_string(),
                                    ::eres_abi::IntoRuntimeValue::into_runtime_value(#field_idents)?,
                                )
                            ),*
                        ]),
                    })
                });
                from_arms.push(quote! {
                    #vname => match payload {
                        ::eres_abi::EnumPayload::Struct(fields) => {
                            let mut map = ::std::collections::BTreeMap::new();
                            for (name, value) in fields {
                                map.insert(name.as_str(), value);
                            }
                            Ok(Self::#vident {
                                #(
                                    #field_idents: <#field_types as ::eres_abi::FromRuntimeValue>::from_runtime_value(
                                        map.remove(#field_names)
                                            .ok_or_else(|| format!("missing field `{}`", #field_names))?
                                    )?,
                                )*
                            })
                        }
                        other => Err(format!("expected struct payload for {}, found {:?}", #vname, other)),
                    }
                });
            }
        }
    }

    quote! {
        impl #ident {
            fn __eres_named_type() -> ::eres_abi::NamedTypeRef {
                ::eres_abi::NamedTypeRef::new(
                    ::std::any::type_name::<Self>(),
                    stringify!(#ident),
                    ::eres_abi::NamedTypeKind::Enum,
                )
            }
        }

        impl ::eres_abi::EresAbi for #ident {
            fn abi_type() -> ::eres_abi::AbiType {
                ::eres_abi::AbiType::Named(Self::__eres_named_type())
            }

            fn descriptors() -> Vec<::eres_abi::TypeDescriptor> {
                let mut registry = ::eres_abi::DescriptorRegistry::default();
                #(registry.extend(<#nested_types as ::eres_abi::EresAbi>::descriptors());)*
                registry.insert(::eres_abi::TypeDescriptor {
                    named: Self::__eres_named_type(),
                    kind: ::eres_abi::TypeDescriptorKind::Enum {
                        variants: vec![#(#descriptor_variants),*],
                    },
                });
                registry.finish()
            }
        }

        impl ::eres_abi::IntoRuntimeValue for #ident {
            fn into_runtime_value(self) -> Result<::eres_abi::RuntimeValue, String> {
                match self {
                    #(#into_arms),*
                }
            }
        }

        impl ::eres_abi::FromRuntimeValue for #ident {
            fn from_runtime_value(value: &::eres_abi::RuntimeValue) -> Result<Self, String> {
                match value {
                    ::eres_abi::RuntimeValue::Enum { named, variant, payload } => {
                        let expected = Self::__eres_named_type();
                        if named.type_id != expected.type_id {
                            return Err(format!(
                                "expected enum type {}, found {}",
                                expected.name,
                                named.name
                            ));
                        }
                        match variant.as_str() {
                            #(#from_arms),*,
                            other => Err(format!("unknown enum variant `{}`", other)),
                        }
                    }
                    other => Err(format!("expected enum runtime value, found {:?}", other)),
                }
            }
        }

        impl ::eres_abi::ComplexEresAbi for #ident {}
    }
}
