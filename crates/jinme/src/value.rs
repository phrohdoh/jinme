use crate::{prelude::*};
use ::std::{fmt, sync::Arc};

mod from;
pub mod optics;
pub mod partials;

/// Alias for `Arc<Value>` - a reference-counted pointer to a Value.
///
/// This is the primary way Values are passed around in the interpreter to avoid
/// unnecessary cloning while maintaining thread safety through Arc.
pub type PtrValue = Arc<Value>;

/// The core runtime value type representing all possible values in the interpreter.
///
/// Each variant carries optional metadata in the form of an `Option<Arc<Map>>`.
/// This metadata can be used to attach source position information, documentation,
/// or other runtime annotations to values.
///
/// # Variants
///
/// - `Nil`: The empty value, equivalent to `nil` in Clojure
/// - `Boolean`: Boolean values (`true` or `false`)
/// - `Integer`: 64-bit signed integers
/// - `Float`: Floating-point numbers (see [`Float`](crate::float::Float))
/// - `String`: UTF-8 strings
/// - `Symbol`: Symbols for namespaced identifiers (see [`Symbol`](crate::symbol::Symbol))
/// - `Keyword`: Keywords for metadata and attributes (see [`Keyword`](crate::keyword::Keyword))
/// - `List`: Linked lists (see [`List`](crate::list::List))
/// - `Vector`: Vectors (see [`Vector`](crate::vector::Vector))
/// - `Set`: Sets (see [`Set`](crate::set::Set))
/// - `Map`: Maps (see [`Map`](crate::map::Map))
/// - `Var`: References to namespace-bound variables (see [`Var`](crate::var::Var))
/// - `Function`: Functions (see [`Function`](crate::function::Function))
/// - `Handle`: Handles for external resources (see [`Handle`](crate::handle::Handle))
///
/// # Metadata
///
/// All variants except `Nil` carry optional metadata via the second field.
/// This is typically used for source position tracking during parsing.
///
/// # Examples
///
/// Creating values:
/// ```
/// # use jinme::prelude::*;
/// let nil_val = Value::nil();
/// let int_val = Value::integer(42);
/// let str_val = Value::string("hello".to_string());
/// let sym_val = Value::symbol_unqualified("my-symbol");
/// ```
///
/// Type checking:
/// ```
/// # use jinme::prelude::*;
/// let val = Value::integer(42);
/// assert!(val.is_integer());
/// assert!(!val.is_string());
/// ```
#[derive(Hash, Ord, PartialOrd, Eq, Clone)]
pub enum Value {
    /// The empty value (`nil` in Clojure)
    Nil(Option<Arc<Map>>),
    /// Boolean values (`true` or `false`)
    Boolean(bool, Option<Arc<Map>>),
    /// 64-bit signed integers
    Integer(i64, Option<Arc<Map>>),
    /// Floating-point numbers
    Float(Float, Option<Arc<Map>>),
    /// UTF-8 strings
    String(String, Option<Arc<Map>>),
    /// Symbols for namespaced identifiers
    Symbol(Symbol, Option<Arc<Map>>),
    /// Keywords for metadata and attributes
    Keyword(Keyword, Option<Arc<Map>>),
    /// Linked lists
    List(List, Option<Arc<Map>>),
    /// Vectors
    Vector(Vector, Option<Arc<Map>>),
    /// Sets
    Set(Set, Option<Arc<Map>>),
    /// Maps
    Map(Map, Option<Arc<Map>>),
    /// References to namespace-bound variables
    Var(PtrVar, Option<Arc<Map>>),
    /// Functions
    Function(PtrFunction, Option<Arc<Map>>),
    /// Handles for external resources
    Handle(Handle, Option<Arc<Map>>),
}

impl Value {
    pub fn view_integer(&self) -> i64 { match self { Self::Integer(integer, _) => integer.to_owned(), _ => panic!("Expected integer value, found {}", self), } }
    pub fn preview_integer(&self) -> Option<i64> { optics::preview_integer(self) }

    pub fn view_float(&self) -> Float { match self { Self::Float(float, _) => float.to_owned(), _ => panic!("Expected float value, found {}", self), } }
    pub fn preview_float(&self) -> Option<Float> { optics::preview_float(self) }

    pub fn view_string(&self) -> String { match self { Self::String(string, _) => string.to_owned(), _ => panic!("Expected string value, found {}", self), } }
    pub fn view_string_ref(&self) -> &str { match self { Self::String(string, _) => string, _ => panic!("Expected string value, found {}", self), } }
    pub fn preview_string(&self) -> Option<String> { optics::preview_string(self) }
    pub fn preview_string_ref(&self) -> Option<&str> { optics::preview_string_ref(self) }

    pub fn view_list(&self) -> List { match self { Self::List(list, _) => list.to_owned(), _ => panic!("Expected list value, found {}", self), } }
    pub fn view_list_ref(&self) -> &List { match self { Self::List(list, _) => list, _ => panic!("Expected list value, found {}", self), } }
    pub fn preview_list(&self) -> Option<List> { optics::preview_list(self) }
    pub fn preview_list_ref(&self) -> Option<&List> { optics::preview_list_ref(self) }

    pub fn view_vector(&self) -> Vector { match self { Self::Vector(vector, _) => vector.to_owned(), _ => panic!("Expected vector value, found {}", self), } }
    pub fn view_vector_ref(&self) -> &Vector { match self { Self::Vector(vector, _) => vector, _ => panic!("Expected vector value, found {}", self), } }
    pub fn preview_vector(&self) -> Option<Vector> { optics::preview_vector(self) }
    pub fn preview_vector_ref(&self) -> Option<&Vector> { optics::preview_vector_ref(self) }

    pub fn view_set(&self) -> Set { match self { Self::Set(set, _) => set.to_owned(), _ => panic!("Expected set value, found {}", self), } }
    pub fn view_set_ref(&self) -> &Set { match self { Self::Set(set, _) => set, _ => panic!("Expected set value, found {}", self), } }
    pub fn preview_set(&self) -> Option<Set> { optics::preview_set(self) }
    pub fn preview_set_ref(&self) -> Option<&Set> { optics::preview_set_ref(self) }

    pub fn view_map(&self) -> Map { match self { Self::Map(map, _) => map.to_owned(), _ => panic!("Expected map value, found {}", self), } }
    pub fn view_map_ref(&self) -> &Map { match self { Self::Map(map, _) => map, _ => panic!("Expected map value, found {}", self), } }
    pub fn preview_map(&self) -> Option<Map> { optics::preview_map(self) }
    pub fn preview_map_ref(&self) -> Option<&Map> { optics::preview_map_ref(self) }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil(_), Self::Nil(_)) => true,
            (Self::Boolean(lhs, _), Self::Boolean(rhs, _)) => lhs == rhs,
            (Self::Integer(lhs, _), Self::Integer(rhs, _)) => lhs == rhs,
            (Self::Float(lhs, _), Self::Float(rhs, _)) => lhs == rhs,
            (Self::String(lhs, _), Self::String(rhs, _)) => lhs == rhs,
            (Self::Symbol(lhs, _), Self::Symbol(rhs, _)) => lhs == rhs,
            (Self::Keyword(lhs, _), Self::Keyword(rhs, _)) => lhs == rhs,
            (Self::List(lhs, _), Self::List(rhs, _)) => lhs == rhs,
            (Self::Vector(lhs, _), Self::Vector(rhs, _)) => lhs == rhs,
            (Self::Set(lhs, _), Self::Set(rhs, _)) => lhs == rhs,
            (Self::Map(lhs, _), Self::Map(rhs, _)) => lhs == rhs,
            (Self::Var(lhs, _), Self::Var(rhs, _)) => lhs == rhs,
            (Self::Function(lhs, _), Self::Function(rhs, _)) => lhs == rhs,
            (Self::Handle(lhs, _), Self::Handle(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Value {
    /// Returns `true` if this value is `nil`
    #[inline]
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil(..))
    }

    /// Returns `true` if this value is a boolean
    #[inline]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(..))
    }

    /// Returns `true` if this value is an integer
    #[inline]
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
    }

    /// Returns `true` if this value is a float
    #[inline]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }

    /// Returns `true` if this value is a string
    #[inline]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    /// Returns `true` if this value is a symbol
    #[inline]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(..))
    }

    /// Returns `true` if this value is a keyword
    #[inline]
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(..))
    }

    /// Returns `true` if this value is a list
    #[inline]
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(..))
    }

    /// Returns `true` if this value is a vector
    #[inline]
    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(..))
    }

    /// Returns `true` if this value is a set
    #[inline]
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(..))
    }

    /// Returns `true` if this value is a map
    #[inline]
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(..))
    }

    /// Returns `true` if this value is a Var reference
    #[inline]
    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var(..))
    }

    /// Returns `true` if this value is a function
    #[inline]
    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(..))
    }

    /// Returns `true` if this value is a handle
    #[inline]
    pub fn is_handle(&self) -> bool {
        matches!(self, Self::Handle(..))
    }

    /// Creates a `nil` value
    #[inline]
    pub fn nil() -> Self {
        Self::Nil(None)
    }

    /// Creates a boolean value
    #[inline]
    pub fn boolean(boolean: bool) -> Self {
        Self::Boolean(boolean, None)
    }

    /// Creates an integer value
    #[inline]
    pub fn integer(integer: i64) -> Self {
        Self::Integer(integer, None)
    }

    /// Creates a float value
    #[inline]
    pub fn float(float: Float) -> Self {
        Self::Float(float, None)
    }

    /// Creates a string value
    #[inline]
    pub fn string(string: String) -> Self {
        Self::String(string, None)
    }

    /// Creates a symbol value from an unqualified symbol name
    #[inline]
    pub fn symbol(symbol: Symbol) -> Self {
        Self::Symbol(symbol, None)
    }

    /// Creates an unqualified symbol value
    #[inline]
    pub fn symbol_unqualified(name: &str) -> Self {
        Self::Symbol(Symbol::new_unqualified(name), None)
    }

    /// Creates a qualified symbol value with namespace and name
    #[inline]
    pub fn symbol_qualified(namespace: &str, name: &str) -> Self {
        Self::Symbol(Symbol::new_qualified(namespace, name), None)
    }

    /// Creates a keyword value
    #[inline]
    pub fn keyword(keyword: Keyword) -> Self {
        Self::Keyword(keyword, None)
    }

    /// Creates an unqualified keyword value
    #[inline]
    pub fn keyword_unqualified(name: &str) -> Self {
        Self::Keyword(Keyword::new_unqualified(name), None)
    }

    /// Creates a qualified keyword value with namespace and name
    #[inline]
    pub fn keyword_qualified(namespace: &str, name: &str) -> Self {
        Self::Keyword(Keyword::new_qualified(namespace, name), None)
    }

    /// Creates a list value
    #[inline]
    pub fn list(list: List) -> Self {
        Self::List(list, None)
    }

    /// Creates a list value from a vector of PtrValue items
    #[inline]
    pub fn list_from(items: Vec<PtrValue>) -> Self {
        Self::List(List::from(items), None)
    }

    /// Creates a vector value
    #[inline]
    pub fn vector(vector: Vector) -> Self {
        Self::Vector(vector, None)
    }
    pub fn vector_from(items: Vec<PtrValue>) -> Self {
        Self::Vector(Vector::from(items), None)
    }
    pub fn set(set: Set) -> Self {
        Self::Set(set, None)
    }
    pub fn set_from(items: Vec<PtrValue>) -> Self {
        Self::Set(Set::new(items), None)
    }
    pub fn map(map: Map) -> Self {
        Self::Map(map, None)
    }
    pub fn map_from(pairs: Vec<(PtrValue, PtrValue)>) -> Self {
        Self::Map(Map::new(pairs), None)
    }
    pub fn var(var: PtrVar) -> Self {
        Self::Var(var.clone(), var.meta())
    }
    pub fn function(function: PtrFunction) -> Self {
        Self::Function(function, None)
    }
    pub fn handle(handle: Handle) -> Self {
        Self::Handle(handle, None)
    }
}

impl Value {
    pub fn nil_ptr() -> PtrValue {
        Arc::new(Self::Nil(None))
    }
    pub fn boolean_ptr(boolean: bool) -> PtrValue {
        Arc::new(Self::Boolean(boolean, None))
    }
    pub fn integer_ptr(integer: i64) -> PtrValue {
        Arc::new(Self::Integer(integer, None))
    }
    pub fn float_ptr(float: Float) -> PtrValue {
        Arc::new(Self::Float(float, None))
    }
    pub fn string_ptr(string: String) -> PtrValue {
        Arc::new(Self::String(string, None))
    }
    pub fn symbol_ptr(symbol: Symbol) -> PtrValue {
        Arc::new(Self::Symbol(symbol, None))
    }
    pub fn symbol_unqualified_ptr(name: &str) -> PtrValue {
        Arc::new(Self::Symbol(Symbol::new_unqualified(name), None))
    }
    pub fn symbol_qualified_ptr(namespace: &str, name: &str) -> PtrValue {
        Arc::new(Self::Symbol(Symbol::new_qualified(namespace, name), None))
    }
    pub fn keyword_ptr(keyword: Keyword) -> PtrValue {
        Arc::new(Self::Keyword(keyword, None))
    }
    pub fn keyword_unqualified_ptr(name: &str) -> PtrValue {
        Arc::new(Self::Keyword(Keyword::new_unqualified(name), None))
    }
    pub fn keyword_qualified_ptr(namespace: &str, name: &str) -> PtrValue {
        Arc::new(Self::Keyword(Keyword::new_qualified(namespace, name), None))
    }
    pub fn list_ptr(list: List) -> PtrValue {
        Arc::new(Self::List(list, None))
    }
    pub fn vector_ptr(vector: Vector) -> PtrValue {
        Arc::new(Self::Vector(vector, None))
    }
    pub fn set_ptr(set: Set) -> PtrValue {
        Arc::new(Self::Set(set, None))
    }
    pub fn map_ptr(map: Map) -> PtrValue {
        Arc::new(Self::Map(map, None))
    }
    pub fn var_ptr(var: PtrVar) -> PtrValue {
        Arc::new(Self::Var(var.clone(), var.meta()))
    }
    pub fn function_ptr(function: PtrFunction) -> PtrValue {
        Arc::new(Self::Function(function, None))
    }
    pub fn handle_ptr(handle: Handle) -> PtrValue {
        Arc::new(Self::Handle(handle, None))
    }
}

impl Value {
    pub fn into_value_ptr(self) -> PtrValue {
        Arc::new(self)
    }
}

// List functions
impl Value {
    pub fn with_meta(&self, meta: Option<Arc<Map>>) -> Self {
        match self {
            Value::Nil(_) => Value::Nil(meta),
            Value::Boolean(boolean, _) => Value::Boolean(boolean.to_owned(), meta),
            Value::Integer(integer, _) => Value::Integer(integer.to_owned(), meta),
            Value::Float(float, _) => Value::Float(float.to_owned(), meta),
            Value::String(string, _) => Value::String(string.to_owned(), meta),
            Value::Symbol(symbol, _) => Value::Symbol(symbol.to_owned(), meta),
            Value::Keyword(keyword, _) => Value::Keyword(keyword.to_owned(), meta),
            Value::List(list, _) => Value::List(list.to_owned(), meta),
            Value::Vector(vector, _) => Value::Vector(vector.to_owned(), meta),
            Value::Set(set, _) => Value::Set(set.to_owned(), meta),
            Value::Map(map, _) => Value::Map(map.to_owned(), meta),
            Value::Var(var, _) => Value::Var(var.to_owned(), meta),
            Value::Function(function, _) => Value::Function(function.to_owned(), meta),
            Value::Handle(handle, _) => Value::Handle(handle.to_owned(), meta),
        }
    }

    pub fn with_meta_ptr(&self, meta: Option<Arc<Map>>) -> PtrValue {
        Arc::new(self.with_meta(meta))
    }

    pub fn new_list_empty() -> Self {
        Self::List(List::new_empty(), None)
    }

    pub fn new_list_empty_ptr() -> PtrValue {
        Arc::new(Self::List(List::new_empty(), None))
    }

    pub fn new_list(elements: Vec<PtrValue>) -> Self {
        Self::List(List::from(elements), None)
    }

    pub fn new_list_ptr(elements: Vec<PtrValue>) -> PtrValue {
        Arc::new(Self::List(List::from(elements), None))
    }
}

// Vector functions
impl Value {
    pub fn new_vector_empty() -> Self {
        Self::Vector(Vector::new_empty(), None)
    }

    pub fn new_vector_empty_ptr() -> PtrValue {
        Arc::new(Self::Vector(Vector::new_empty(), None))
    }

    pub fn new_vector(elements: Vec<PtrValue>) -> Self {
        Self::Vector(Vector::from(elements), None)
    }

    pub fn new_vector_ptr(elements: Vec<PtrValue>) -> PtrValue {
        Arc::new(Self::Vector(Vector::from(elements), None))
    }
}

// Set functions
impl Value {
    pub fn new_set_empty() -> Self {
        Self::Set(Set::new_empty(), None)
    }

    pub fn new_set_empty_ptr() -> PtrValue {
        Arc::new(Self::Set(Set::new_empty(), None))
    }

    pub fn new_set(elements: Vec<PtrValue>) -> Self {
        Self::Set(Set::new(elements), None)
    }

    pub fn new_set_ptr(elements: Vec<PtrValue>) -> PtrValue {
        Arc::new(Self::Set(Set::new(elements), None))
    }
}

// Map functions
impl Value {
    pub fn new_map_empty() -> Self {
        Self::Map(Map::new_empty(), None)
    }

    pub fn new_map_empty_ptr() -> PtrValue {
        Arc::new(Self::Map(Map::new_empty(), None))
    }

    pub fn new_map(elements: Vec<(PtrValue, PtrValue)>) -> Self {
        Self::Map(Map::new(elements), None)
    }

    pub fn new_map_ptr(elements: Vec<(PtrValue, PtrValue)>) -> PtrValue {
        Arc::new(Self::Map(Map::new(elements), None))
    }
}

// #[derive(Debug)]
// pub enum GetFunctionError {
//     ValueIsNotFunction,
// }

#[derive(Debug)]
pub enum GetHandleError {
    IncorrectValueType,
    IncorrectHandleType,
}

impl Value {
    pub fn try_get_handle<T>(self: &'_ Value) -> Result<T, GetHandleError>
    where
        T: IHandle + Clone + 'static,
    {
        value::optics::preview_handle(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| {
                handle
                    .downcast_ref::<T>()
                    .map(|ref_t| ref_t.to_owned())
                    .ok_or(GetHandleError::IncorrectHandleType)
            })
    }

    pub fn try_get_handle_ref<T>(&self) -> Result<T, GetHandleError>
    where
        T: IHandle + Clone + 'static,
    {
        value::optics::preview_handle_ref(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| {
                handle
                    .downcast_ref::<T>()
                    .ok_or(GetHandleError::IncorrectHandleType)
            })
    }

    pub fn try_get_handle_mut<T: IHandle + Clone + 'static>(&self) -> Result<T, GetHandleError> {
        value::optics::preview_handle_ref(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| {
                handle
                    .downcast_ref::<T>()
                    .ok_or(GetHandleError::IncorrectHandleType)
            })
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(_meta) => write!(f, "Value::Nil"),
            Self::Boolean(boolean, _meta) => write!(f, "Value::Boolean({})", boolean),
            Self::Integer(integer, _meta) => write!(f, "Value::Integer({})", integer),
            Self::Float(float, _meta) => write!(f, "Value::Float({:?})", float),
            Self::String(string, _meta) => write!(f, "Value::String({:?})", string),
            Self::Symbol(symbol, _meta) => write!(f, "Value::Symbol({:?})", symbol),
            Self::Keyword(keyword, _meta) => write!(f, "Value::Keyword({:?})", keyword),
            Self::List(list, _meta) => write!(f, "Value::List({:?})", list),
            Self::Vector(vector, _meta) => write!(f, "Value::Vector({:?})", vector),
            Self::Set(set, _meta) => write!(f, "Value::Set({:?})", set),
            Self::Map(map, _meta) => write!(f, "Value::Map({:?})", map),
            Self::Var(var, _meta) => {
                write!(f, "Value::Var({:p})", PtrVar::as_ptr(var).cast::<()>())
            }
            Self::Function(func, _meta) => write!(f, "Value::Function({:?})", func),
            Self::Handle(handle, _meta) => write!(f, "Value::Handle({:?})", handle),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(_meta) => write!(f, "nil"),
            Self::Boolean(boolean, _meta) => write!(f, "{}", boolean),
            Self::Integer(integer, _meta) => write!(f, "{}", integer),
            Self::Float(float, _meta) => write!(f, "{}", float),
            Self::String(string, _meta) => write!(f, "\"{}\"", string),
            Self::Symbol(symbol, _meta) => write!(f, "{}", symbol),
            Self::Keyword(keyword, _meta) => write!(f, "{}", keyword),
            Self::List(list, _meta) => write!(f, "{}", list),
            Self::Vector(vector, _meta) => write!(f, "{}", vector),
            Self::Set(set, _meta) => write!(f, "{}", set),
            Self::Map(map, _meta) => write!(f, "{}", map),
            Self::Var(var, _meta) => write!(f, "#var[{:p}]", PtrVar::as_ptr(var).cast::<()>()),
            Self::Function(func, _meta) => write!(
                f,
                "#fn[{addr:p} {name} {arities}]",
                addr = PtrFunction::as_ptr(func).cast::<()>(),
                name = func
                    .name()
                    .map(|s| format!("\"{}\"", s))
                    .unwrap_or("<unnamed>".to_owned()),
                arities = Value::vector_from(
                    func.arity_strings()
                        .into_iter()
                        .map(|s| Arc::new(Value::string(s)))
                        .collect(),
                )
            ),
            Self::Handle(handle, _meta) => write!(f, "{:?}", handle),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn create_env() -> PtrEnvironment {
        let mut env_builder = Environment::builder();
        env_builder.set_current_namespace_var("clojure.core", "*ns*");
        env_builder.insert_namespace(Namespace::new_empty_ptr("clojure.core"));
        env_builder.build_ptr()
    }

    #[test]
    fn unqualified_keyword_equality() {
        let env = create_env();
        let k1 = read(env.clone(), ":foo").unwrap().1.unwrap();
        let k2 = read(env.clone(), " :foo").unwrap().1.unwrap();
        assert_eq!(k1, k2);
    }
}
