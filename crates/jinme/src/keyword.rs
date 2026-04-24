/// A keyword that evaluates to itself.
///
/// Keywords are used for metadata, attributes, and as keys in maps. They may be
/// either unqualified (just a name) or qualified (with a namespace).
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let kw = Keyword::new_qualified("user", "my-key");
/// assert_eq!(kw.namespace(), Some("user"));
/// assert_eq!(kw.name(), "my-key");
/// ```
use std::fmt;

pub mod optics;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Keyword {
    /// An unqualified keyword (just a name)
    Unqualified(KeywordUnqualified),
    /// A qualified keyword with namespace and name
    Qualified(KeywordQualified),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeywordUnqualified(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeywordQualified(String, String);

impl Keyword {
    pub fn new_unqualified(name: &str) -> Self {
        Self::Unqualified(KeywordUnqualified(name.to_owned()))
    }

    pub fn new_qualified(namespace: &str, name: &str) -> Self {
        Self::Qualified(KeywordQualified(namespace.to_owned(), name.to_owned()))
    }

    pub fn is_unqualified(&self) -> bool {
        matches!(self, Self::Unqualified(_))
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified(_))
    }

    pub fn as_unqualified_keyword(&self) -> Option<&KeywordUnqualified> {
        match self {
            Self::Unqualified(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn as_qualified_keyword(&self) -> Option<&KeywordQualified> {
        match self {
            Self::Qualified(inner) => Some(inner),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Unqualified(keyword) => keyword.name(),
            Self::Qualified(keyword) => keyword.name(),
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Unqualified(_) => None,
            Self::Qualified(keyword) => Some(keyword.namespace()),
        }
    }

    pub fn namespace_or_panic(&self) -> &str {
        self.namespace().expect(&format!(
            "attempted to get namespace of unqualified keyword: {}",
            self
        ))
    }
}

impl KeywordUnqualified {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }

    pub fn name(&self) -> &str {
        self.0.as_str()
    }
}

impl KeywordQualified {
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

impl From<KeywordUnqualified> for Keyword {
    fn from(keyword: KeywordUnqualified) -> Self {
        Self::Unqualified(keyword)
    }
}

impl From<KeywordQualified> for Keyword {
    fn from(keyword: KeywordQualified) -> Self {
        Self::Qualified(keyword)
    }
}

impl From<&KeywordUnqualified> for Keyword {
    fn from(keyword: &KeywordUnqualified) -> Self {
        Self::new_unqualified(keyword.name())
    }
}

impl From<&KeywordQualified> for Keyword {
    fn from(keyword: &KeywordQualified) -> Self {
        Self::new_qualified(keyword.namespace(), keyword.name())
    }
}

impl From<Keyword> for Option<KeywordUnqualified> {
    fn from(keyword: Keyword) -> Self {
        match keyword {
            Keyword::Unqualified(keyword) => Some(keyword),
            _ => None,
        }
    }
}

impl From<Keyword> for Option<KeywordQualified> {
    fn from(keyword: Keyword) -> Self {
        match keyword {
            Keyword::Qualified(keyword) => Some(keyword),
            _ => None,
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Unqualified(keyword) => write!(f, "{keyword}"),
            Keyword::Qualified(keyword) => write!(f, "{keyword}"),
        }
    }
}

impl fmt::Display for KeywordUnqualified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":{}", self.0)
    }
}

impl fmt::Display for KeywordQualified {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":{}/{}", self.0, self.1)
    }
}
