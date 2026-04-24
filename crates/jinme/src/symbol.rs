/// An identifier that evaluates to a `Value`.
///
/// Symbols may be either unqualified (just a name) or qualified (with a namespace).
/// Unqualified symbols are resolved in the current namespace, while qualified symbols
/// specify both the namespace and the name.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let sym = Symbol::new_qualified("user", "my-var");
/// assert_eq!(sym.namespace(), Some("user"));
/// assert_eq!(sym.name(), "my-var");
/// ```
use std::fmt;

pub mod optics;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    /// An unqualified symbol (just a name)
    Unqualified(SymbolUnqualified),
    /// A qualified symbol with namespace and name
    Qualified(SymbolQualified),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolUnqualified(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymbolQualified(String, String);

impl Symbol {
    pub fn new_unqualified(name: &str) -> Self {
        Self::Unqualified(SymbolUnqualified(name.to_owned()))
    }

    pub fn new_qualified(namespace: &str, name: &str) -> Self {
        Self::Qualified(SymbolQualified(namespace.to_owned(), name.to_owned()))
    }

    pub fn is_unqualified(&self) -> bool {
        matches!(self, Self::Unqualified(_))
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified(_))
    }

    pub fn as_unqualified_symbol(&self) -> Option<&SymbolUnqualified> {
        match self {
            Self::Unqualified(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_qualified_symbol(&self) -> Option<&SymbolQualified> {
        match self {
            Self::Qualified(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Unqualified(symbol) => symbol.name(),
            Self::Qualified(symbol) => symbol.name(),
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Unqualified(_) => None,
            Self::Qualified(symbol) => Some(symbol.namespace()),
        }
    }

    pub fn namespace_or_panic(&self) -> &str {
        self.namespace().expect(&format!(
            "attempted to get namespace of unqualified symbol: {}",
            self
        ))
    }
}

impl SymbolUnqualified {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }
}

impl SymbolQualified {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self(namespace.to_owned(), name.to_owned())
    }

    pub fn namespace(&self) -> &str {
        self.0.as_str()
    }

    pub fn name(&self) -> &str {
        self.1.as_str()
    }
}

impl From<&str> for SymbolUnqualified {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for SymbolUnqualified {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl<NS, N> From<(NS, N)> for SymbolQualified
where
    NS: Into<SymbolUnqualified>,
    N: Into<SymbolUnqualified>,
{
    fn from((ns_name, name): (NS, N)) -> Self {
        Self::new(ns_name.into().name(), name.into().name())
    }
}

impl From<SymbolUnqualified> for Symbol {
    fn from(symbol: SymbolUnqualified) -> Self {
        Self::Unqualified(symbol)
    }
}

impl From<SymbolQualified> for Symbol {
    fn from(symbol: SymbolQualified) -> Self {
        Self::Qualified(symbol)
    }
}

impl From<&SymbolUnqualified> for Symbol {
    fn from(symbol: &SymbolUnqualified) -> Self {
        Self::new_unqualified(symbol.name())
    }
}

impl From<&SymbolQualified> for Symbol {
    fn from(symbol: &SymbolQualified) -> Self {
        Self::new_qualified(symbol.namespace(), symbol.name())
    }
}

impl From<Symbol> for Option<SymbolUnqualified> {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Unqualified(symbol) => Some(symbol),
            _ => None,
        }
    }
}

impl From<Symbol> for Option<SymbolQualified> {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Qualified(symbol) => Some(symbol),
            _ => None,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Unqualified(symbol) => write!(f, "{symbol}"),
            Symbol::Qualified(symbol) => write!(f, "{symbol}"),
        }
    }
}

impl fmt::Display for SymbolUnqualified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for SymbolQualified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
