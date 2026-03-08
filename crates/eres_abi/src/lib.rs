use std::collections::BTreeMap;

use wasmtime::{Val, ValType};

pub use eres_abi_macros::EresAbi;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NamedTypeKind {
    Struct,
    Enum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedTypeRef {
    pub type_id: String,
    pub name: String,
    pub kind: NamedTypeKind,
}

impl NamedTypeRef {
    pub fn new(
        type_id: impl Into<String>,
        name: impl Into<String>,
        kind: NamedTypeKind,
    ) -> Self {
        Self {
            type_id: type_id.into(),
            name: name.into(),
            kind,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbiType {
    Int,
    Float,
    Bool,
    String,
    List(Box<AbiType>),
    Tuple(Vec<AbiType>),
    Named(NamedTypeRef),
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub name: String,
    pub ty: AbiType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariantDescriptor {
    Unit {
        name: String,
    },
    Tuple {
        name: String,
        ty: AbiType,
    },
    Struct {
        name: String,
        fields: Vec<FieldDescriptor>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDescriptorKind {
    Struct {
        fields: Vec<FieldDescriptor>,
    },
    Enum {
        variants: Vec<EnumVariantDescriptor>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDescriptor {
    pub named: NamedTypeRef,
    pub kind: TypeDescriptorKind,
}

#[derive(Debug, Default, Clone)]
pub struct DescriptorRegistry {
    descriptors: BTreeMap<String, TypeDescriptor>,
}

impl DescriptorRegistry {
    pub fn insert(&mut self, descriptor: TypeDescriptor) {
        self.descriptors
            .entry(descriptor.named.type_id.clone())
            .or_insert(descriptor);
    }

    pub fn extend<I>(&mut self, descriptors: I)
    where
        I: IntoIterator<Item = TypeDescriptor>,
    {
        for descriptor in descriptors {
            self.insert(descriptor);
        }
    }

    pub fn finish(self) -> Vec<TypeDescriptor> {
        self.descriptors.into_values().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List {
        element_type: AbiType,
        elements: Vec<RuntimeValue>,
    },
    Tuple {
        element_types: Vec<AbiType>,
        elements: Vec<RuntimeValue>,
    },
    Struct {
        named: NamedTypeRef,
        fields: Vec<(String, RuntimeValue)>,
    },
    Enum {
        named: NamedTypeRef,
        variant: String,
        payload: EnumPayload,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumPayload {
    Unit,
    Tuple(Box<RuntimeValue>),
    Struct(Vec<(String, RuntimeValue)>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeValueHandle(pub u32);

#[derive(Debug, Default, Clone)]
pub struct RuntimeHeap {
    values: Vec<RuntimeValue>,
}

impl RuntimeHeap {
    pub fn allocate(&mut self, value: RuntimeValue) -> RuntimeValueHandle {
        let index = self.values.len() as u32;
        self.values.push(value);
        RuntimeValueHandle(index)
    }

    pub fn get(&self, handle: RuntimeValueHandle) -> Result<&RuntimeValue, String> {
        self.values
            .get(handle.0 as usize)
            .ok_or_else(|| format!("invalid runtime handle {}", handle.0))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HostValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Handle(RuntimeValueHandle),
}

pub trait EresAbi {
    fn abi_type() -> AbiType;

    fn descriptors() -> Vec<TypeDescriptor> {
        Vec::new()
    }
}

pub trait IntoRuntimeValue: EresAbi {
    fn into_runtime_value(self) -> Result<RuntimeValue, String>;
}

pub trait FromRuntimeValue: EresAbi + Sized {
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String>;
}

pub trait ComplexEresAbi: EresAbi + IntoRuntimeValue + FromRuntimeValue {}

pub trait IntoEresValue: EresAbi {
    fn into_eres_value(self, heap: &mut RuntimeHeap) -> Result<HostValue, String>;
}

pub trait FromEresValue: EresAbi + Sized {
    fn from_eres_value(value: &HostValue, heap: &RuntimeHeap) -> Result<Self, String>;
}

pub trait IntoEresReturn: EresAbi {
    fn into_eres_return(self, heap: &mut RuntimeHeap) -> Result<Option<HostValue>, String>;
}

impl<T> IntoEresValue for T
where
    T: ComplexEresAbi,
{
    fn into_eres_value(self, heap: &mut RuntimeHeap) -> Result<HostValue, String> {
        Ok(HostValue::Handle(heap.allocate(self.into_runtime_value()?)))
    }
}

impl<T> FromEresValue for T
where
    T: ComplexEresAbi,
{
    fn from_eres_value(value: &HostValue, heap: &RuntimeHeap) -> Result<Self, String> {
        match value {
            HostValue::Handle(handle) => T::from_runtime_value(heap.get(*handle)?),
            other => Err(format!(
                "expected handle for {}, found {:?}",
                format_abi_type(&T::abi_type()),
                other
            )),
        }
    }
}

impl<T> IntoEresReturn for T
where
    T: IntoEresValue,
{
    fn into_eres_return(self, heap: &mut RuntimeHeap) -> Result<Option<HostValue>, String> {
        self.into_eres_value(heap).map(Some)
    }
}

impl<T> IntoEresReturn for Result<T, String>
where
    T: IntoEresValue,
{
    fn into_eres_return(self, heap: &mut RuntimeHeap) -> Result<Option<HostValue>, String> {
        self.and_then(|value| value.into_eres_return(heap))
    }
}

impl<T> EresAbi for Result<T, String>
where
    T: EresAbi,
{
    fn abi_type() -> AbiType {
        T::abi_type()
    }

    fn descriptors() -> Vec<TypeDescriptor> {
        T::descriptors()
    }
}

impl EresAbi for () {
    fn abi_type() -> AbiType {
        AbiType::Unit
    }
}

impl IntoEresReturn for () {
    fn into_eres_return(self, _heap: &mut RuntimeHeap) -> Result<Option<HostValue>, String> {
        Ok(None)
    }
}

impl IntoEresReturn for Result<(), String> {
    fn into_eres_return(self, _heap: &mut RuntimeHeap) -> Result<Option<HostValue>, String> {
        self.map(|_| None)
    }
}

impl EresAbi for i64 {
    fn abi_type() -> AbiType {
        AbiType::Int
    }
}

impl IntoRuntimeValue for i64 {
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Int(self))
    }
}

impl FromRuntimeValue for i64 {
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Int(value) => Ok(*value),
            other => Err(format!("expected Int runtime value, found {:?}", other)),
        }
    }
}

impl IntoEresValue for i64 {
    fn into_eres_value(self, _heap: &mut RuntimeHeap) -> Result<HostValue, String> {
        Ok(HostValue::Int(self))
    }
}

impl FromEresValue for i64 {
    fn from_eres_value(value: &HostValue, _heap: &RuntimeHeap) -> Result<Self, String> {
        match value {
            HostValue::Int(value) => Ok(*value),
            other => Err(format!("expected Int host argument, found {:?}", other)),
        }
    }
}

impl EresAbi for f64 {
    fn abi_type() -> AbiType {
        AbiType::Float
    }
}

impl IntoRuntimeValue for f64 {
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Float(self))
    }
}

impl FromRuntimeValue for f64 {
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Float(value) => Ok(*value),
            other => Err(format!("expected Float runtime value, found {:?}", other)),
        }
    }
}

impl IntoEresValue for f64 {
    fn into_eres_value(self, _heap: &mut RuntimeHeap) -> Result<HostValue, String> {
        Ok(HostValue::Float(self))
    }
}

impl FromEresValue for f64 {
    fn from_eres_value(value: &HostValue, _heap: &RuntimeHeap) -> Result<Self, String> {
        match value {
            HostValue::Float(value) => Ok(*value),
            other => Err(format!("expected Float host argument, found {:?}", other)),
        }
    }
}

impl EresAbi for bool {
    fn abi_type() -> AbiType {
        AbiType::Bool
    }
}

impl IntoRuntimeValue for bool {
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Bool(self))
    }
}

impl FromRuntimeValue for bool {
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Bool(value) => Ok(*value),
            other => Err(format!("expected Bool runtime value, found {:?}", other)),
        }
    }
}

impl IntoEresValue for bool {
    fn into_eres_value(self, _heap: &mut RuntimeHeap) -> Result<HostValue, String> {
        Ok(HostValue::Bool(self))
    }
}

impl FromEresValue for bool {
    fn from_eres_value(value: &HostValue, _heap: &RuntimeHeap) -> Result<Self, String> {
        match value {
            HostValue::Bool(value) => Ok(*value),
            other => Err(format!("expected Bool host argument, found {:?}", other)),
        }
    }
}

impl EresAbi for String {
    fn abi_type() -> AbiType {
        AbiType::String
    }
}

impl IntoRuntimeValue for String {
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::String(self))
    }
}

impl FromRuntimeValue for String {
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::String(value) => Ok(value.clone()),
            other => Err(format!("expected String runtime value, found {:?}", other)),
        }
    }
}

impl ComplexEresAbi for String {}

impl<T> EresAbi for Vec<T>
where
    T: EresAbi,
{
    fn abi_type() -> AbiType {
        AbiType::List(Box::new(T::abi_type()))
    }

    fn descriptors() -> Vec<TypeDescriptor> {
        T::descriptors()
    }
}

impl<T> IntoRuntimeValue for Vec<T>
where
    T: EresAbi + IntoRuntimeValue,
{
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::List {
            element_type: T::abi_type(),
            elements: self
                .into_iter()
                .map(IntoRuntimeValue::into_runtime_value)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl<T> FromRuntimeValue for Vec<T>
where
    T: EresAbi + FromRuntimeValue,
{
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::List {
                element_type,
                elements,
            } => {
                if element_type != &T::abi_type() {
                    return Err(format!(
                        "expected list element type {}, found {}",
                        format_abi_type(&T::abi_type()),
                        format_abi_type(element_type)
                    ));
                }
                elements
                    .iter()
                    .map(T::from_runtime_value)
                    .collect::<Result<Vec<_>, _>>()
            }
            other => Err(format!("expected List runtime value, found {:?}", other)),
        }
    }
}

impl<T> ComplexEresAbi for Vec<T> where T: EresAbi + IntoRuntimeValue + FromRuntimeValue {}

impl<A, B> EresAbi for (A, B)
where
    A: EresAbi,
    B: EresAbi,
{
    fn abi_type() -> AbiType {
        AbiType::Tuple(vec![A::abi_type(), B::abi_type()])
    }

    fn descriptors() -> Vec<TypeDescriptor> {
        let mut registry = DescriptorRegistry::default();
        registry.extend(A::descriptors());
        registry.extend(B::descriptors());
        registry.finish()
    }
}

impl<A, B> IntoRuntimeValue for (A, B)
where
    A: EresAbi + IntoRuntimeValue,
    B: EresAbi + IntoRuntimeValue,
{
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Tuple {
            element_types: vec![A::abi_type(), B::abi_type()],
            elements: vec![self.0.into_runtime_value()?, self.1.into_runtime_value()?],
        })
    }
}

impl<A, B> FromRuntimeValue for (A, B)
where
    A: EresAbi + FromRuntimeValue,
    B: EresAbi + FromRuntimeValue,
{
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Tuple {
                element_types,
                elements,
            } => {
                if element_types != &vec![A::abi_type(), B::abi_type()] || elements.len() != 2 {
                    return Err("tuple runtime value shape mismatch".to_string());
                }
                Ok((
                    A::from_runtime_value(&elements[0])?,
                    B::from_runtime_value(&elements[1])?,
                ))
            }
            other => Err(format!("expected Tuple runtime value, found {:?}", other)),
        }
    }
}

impl<A, B> ComplexEresAbi for (A, B)
where
    A: EresAbi + IntoRuntimeValue + FromRuntimeValue,
    B: EresAbi + IntoRuntimeValue + FromRuntimeValue,
{
}

impl<A, B, C> EresAbi for (A, B, C)
where
    A: EresAbi,
    B: EresAbi,
    C: EresAbi,
{
    fn abi_type() -> AbiType {
        AbiType::Tuple(vec![A::abi_type(), B::abi_type(), C::abi_type()])
    }

    fn descriptors() -> Vec<TypeDescriptor> {
        let mut registry = DescriptorRegistry::default();
        registry.extend(A::descriptors());
        registry.extend(B::descriptors());
        registry.extend(C::descriptors());
        registry.finish()
    }
}

impl<A, B, C> IntoRuntimeValue for (A, B, C)
where
    A: EresAbi + IntoRuntimeValue,
    B: EresAbi + IntoRuntimeValue,
    C: EresAbi + IntoRuntimeValue,
{
    fn into_runtime_value(self) -> Result<RuntimeValue, String> {
        Ok(RuntimeValue::Tuple {
            element_types: vec![A::abi_type(), B::abi_type(), C::abi_type()],
            elements: vec![
                self.0.into_runtime_value()?,
                self.1.into_runtime_value()?,
                self.2.into_runtime_value()?,
            ],
        })
    }
}

impl<A, B, C> FromRuntimeValue for (A, B, C)
where
    A: EresAbi + FromRuntimeValue,
    B: EresAbi + FromRuntimeValue,
    C: EresAbi + FromRuntimeValue,
{
    fn from_runtime_value(value: &RuntimeValue) -> Result<Self, String> {
        match value {
            RuntimeValue::Tuple {
                element_types,
                elements,
            } => {
                if element_types != &vec![A::abi_type(), B::abi_type(), C::abi_type()]
                    || elements.len() != 3
                {
                    return Err("tuple runtime value shape mismatch".to_string());
                }
                Ok((
                    A::from_runtime_value(&elements[0])?,
                    B::from_runtime_value(&elements[1])?,
                    C::from_runtime_value(&elements[2])?,
                ))
            }
            other => Err(format!("expected Tuple runtime value, found {:?}", other)),
        }
    }
}

impl<A, B, C> ComplexEresAbi for (A, B, C)
where
    A: EresAbi + IntoRuntimeValue + FromRuntimeValue,
    B: EresAbi + IntoRuntimeValue + FromRuntimeValue,
    C: EresAbi + IntoRuntimeValue + FromRuntimeValue,
{
}

#[derive(Clone)]
pub struct HostFunction {
    pub name: &'static str,
    pub params: Vec<AbiType>,
    pub result: AbiType,
    pub descriptors: Vec<TypeDescriptor>,
    pub call: fn(&mut RuntimeHeap, &[HostValue]) -> Result<Option<HostValue>, String>,
}

pub fn format_abi_type(ty: &AbiType) -> String {
    match ty {
        AbiType::Int => "Int".to_string(),
        AbiType::Float => "Float".to_string(),
        AbiType::Bool => "Bool".to_string(),
        AbiType::String => "String".to_string(),
        AbiType::List(inner) => format!("List<{}>", format_abi_type(inner)),
        AbiType::Tuple(elements) => format!(
            "({})",
            elements
                .iter()
                .map(format_abi_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        AbiType::Named(named) => named.name.clone(),
        AbiType::Unit => "Unit".to_string(),
    }
}

pub fn abi_type_to_val_type(ty: &AbiType) -> Option<ValType> {
    match ty {
        AbiType::Int => Some(ValType::I64),
        AbiType::Float => Some(ValType::F64),
        AbiType::Bool => Some(ValType::I32),
        AbiType::String | AbiType::List(_) | AbiType::Tuple(_) | AbiType::Named(_) => {
            Some(ValType::I32)
        }
        AbiType::Unit => None,
    }
}

pub fn val_to_host_value(val: &Val, ty: &AbiType) -> Result<HostValue, String> {
    match (val, ty) {
        (Val::I64(value), AbiType::Int) => Ok(HostValue::Int(*value)),
        (Val::F64(value), AbiType::Float) => Ok(HostValue::Float(f64::from_bits(*value))),
        (Val::I32(value), AbiType::Bool) => Ok(HostValue::Bool(*value != 0)),
        (Val::I32(value), AbiType::String | AbiType::List(_) | AbiType::Tuple(_) | AbiType::Named(_)) => {
            Ok(HostValue::Handle(RuntimeValueHandle(*value as u32)))
        }
        other => Err(format!("unsupported wasm value for host bridge: {:?}", other)),
    }
}

pub fn host_value_to_val(value: HostValue, ty: &AbiType) -> Result<Val, String> {
    match (value, ty) {
        (HostValue::Int(value), AbiType::Int) => Ok(Val::I64(value)),
        (HostValue::Float(value), AbiType::Float) => Ok(Val::F64(value.to_bits())),
        (HostValue::Bool(value), AbiType::Bool) => Ok(Val::I32(if value { 1 } else { 0 })),
        (
            HostValue::Handle(handle),
            AbiType::String | AbiType::List(_) | AbiType::Tuple(_) | AbiType::Named(_),
        ) => Ok(Val::I32(handle.0 as i32)),
        (value, ty) => Err(format!(
            "host bridge type mismatch for {}: {:?}",
            format_abi_type(ty),
            value
        )),
    }
}

#[macro_export]
macro_rules! eres_host_function {
    ($rust_fn:path, name = $name:literal, params = [$($param_ty:ty),* $(,)?], result = $result_ty:ty) => {{
        fn wrapper(
            heap: &mut $crate::RuntimeHeap,
            args: &[$crate::HostValue],
        ) -> Result<Option<$crate::HostValue>, String> {
            let mut iter = args.iter();
            let result = $rust_fn(
                $(
                    <$param_ty as $crate::FromEresValue>::from_eres_value(
                        iter.next().ok_or_else(|| format!("host function {} missing argument", $name))?,
                        heap,
                    )?
                ),*
            );
            if iter.next().is_some() {
                return Err(format!("host function {} received too many arguments", $name));
            }
            <$result_ty as $crate::IntoEresReturn>::into_eres_return(result, heap)
        }

        let mut descriptors = $crate::DescriptorRegistry::default();
        $(
            descriptors.extend(<$param_ty as $crate::EresAbi>::descriptors());
        )*
        descriptors.extend(<$result_ty as $crate::EresAbi>::descriptors());

        $crate::HostFunction {
            name: $name,
            params: vec![$(<$param_ty as $crate::EresAbi>::abi_type()),*],
            result: <$result_ty as $crate::EresAbi>::abi_type(),
            descriptors: descriptors.finish(),
            call: wrapper,
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, EresAbi)]
    struct User {
        name: String,
        active: bool,
    }

    #[derive(Debug, Clone, PartialEq, EresAbi)]
    enum Tag {
        Guest,
        Name(String),
        User { id: i64, name: String },
    }

    #[test]
    fn string_roundtrip_uses_handles() {
        let mut heap = RuntimeHeap::default();
        let host = "hi".to_string().into_eres_value(&mut heap).expect("encode string");
        let got = String::from_eres_value(&host, &heap).expect("decode string");
        assert_eq!(got, "hi");
    }

    #[test]
    fn complex_types_roundtrip() {
        let mut heap = RuntimeHeap::default();

        let user = User {
            name: "Ada".to_string(),
            active: true,
        };
        let encoded = user.clone().into_eres_value(&mut heap).expect("encode user");
        let decoded = User::from_eres_value(&encoded, &heap).expect("decode user");
        assert_eq!(decoded, user);

        let tag = Tag::User {
            id: 7,
            name: "Lin".to_string(),
        };
        let encoded = tag.clone().into_eres_value(&mut heap).expect("encode tag");
        let decoded = Tag::from_eres_value(&encoded, &heap).expect("decode tag");
        assert_eq!(decoded, tag);

        let tuple = ("x".to_string(), vec![1_i64, 2_i64], (true, 9_i64));
        let encoded = tuple.clone().into_eres_value(&mut heap).expect("encode tuple");
        let decoded =
            <(String, Vec<i64>, (bool, i64))>::from_eres_value(&encoded, &heap).expect("decode");
        assert_eq!(decoded, tuple);
    }
}
