use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::span::Spanned;


/// Single node of the KDL document
#[derive(Debug, Clone)]
pub struct Node<S> {
    pub type_name: Option<Spanned<TypeName, S>>,
    pub node_name: Spanned<Box<str>, S>,
    pub arguments: Vec<Spanned<Value<S>, S>>,
    pub properties: BTreeMap<Spanned<Box<str>, S>, Value<S>>,
    pub children: Option<Spanned<Vec<Spanned<Node<S>, S>>, S>>,
}

/// KDL document root
#[derive(Debug, Clone)]
pub struct Document<S> {
    pub children: Vec<Spanned<Node<S>, S>>,
}

/// Potentially unlimited size integer value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer(Box<str>);

/// Potentially unlimited precision decimal value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal(Box<str>);

/// Possibly typed value
#[derive(Debug, Clone)]
pub struct Value<S> {
    pub type_name: Option<Spanned<TypeName, S>>,
    pub literal: Spanned<Literal, S>,
}

/// Type identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeName(TypeNameInner);

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeNameInner {
    Builtin(BuiltinType),
    Custom(Box<str>),
}

/// Known type identifier described by specification
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinType {
}

/// Scalar KDL value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// Integer value
    Int(Integer),
    /// Decimal (or floating point) value
    Decimal(Decimal),
    /// String value
    String(Box<str>),
    /// Boolean value
    Boolean(bool),
    /// Null value
    Null,
}

impl<S> Node<S> {
    /// Returns node children
    pub fn children(&self) -> impl Iterator<Item=&Spanned<Node<S>, S>> {
        self.children.iter().flat_map(|c| c.iter())
    }
}

impl BuiltinType {
    fn as_str(&self) -> &'static str {
        match self {
            _ => unreachable!(),
        }
    }
}

impl TypeName {
    // TODO(tailhook) for public API check identifier for validness
    pub(crate) fn from_string(val: Box<str>) -> TypeName {
        match &val[..] {
            _ => TypeName(TypeNameInner::Custom(val)),
        }
    }
}

impl std::ops::Deref for TypeName {
    type Target = str;
    fn deref(&self) -> &str {
        match &self.0 {
            TypeNameInner::Builtin(t) => t.as_str(),
            TypeNameInner::Custom(t) => t.as_ref(),
        }
    }
}

