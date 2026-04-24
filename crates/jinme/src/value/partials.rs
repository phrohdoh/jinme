//! Partialed versions of the `optics2::modify_*` functions.
//!
//! These functions enable functional composition by taking the transformation function first
//! and returning a function that transforms a `Value`. This inverts the argument order compared
//! to the eager versions in `optics2`, allowing you to:
//! - Build pipelines of transformations
//! - Compose modifications using higher-order functions
//! - Create reusable transformation builders
//!
//! Example:
//! ```
//! # use jinme::value::{Value, partials::modify_integer};
//! let double = modify_integer(|x| x * 2);
//! let value = Value::integer_ptr(3);
//! let result = double(value);
//! ```

use crate::float::Float;
use crate::function::PtrFunction;
use crate::handle::Handle;
use crate::keyword::Keyword;
use crate::list::List;
use crate::map::Map;
use crate::set::Set;
use crate::symbol::Symbol;
use crate::value::PtrValue;
use crate::value::optics;
use crate::var::PtrVar;
use crate::vector::Vector;
use ::std::sync::Arc;

/// Creates a partialed modifier that transforms integers.
/// Returns a function that takes a `Value` and modifies it if it is an integer.
pub fn modify_integer(f: impl Fn(i64) -> i64 + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_integer(value, f.clone())
}

/// Creates a partialed modifier for boolean values.
/// Returns a function that takes a `Value` and modifies it if it is a boolean.
pub fn modify_boolean(f: impl Fn(bool) -> bool + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_boolean(value, f.clone())
}

/// Creates a partialed modifier for float values.
/// Returns a function that takes a `Value` and modifies it if it is a float.
pub fn modify_float(f: impl Fn(f64) -> f64 + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_float(value, |float: Float| Float::from(f(float.as_f64())))
}

/// Creates a partialed modifier for string values.
/// Returns a function that takes a `Value` and modifies it if it is a string.
pub fn modify_string(f: impl Fn(String) -> String + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_string(value, f.clone())
}

/// Creates a partialed modifier for symbol values.
/// Returns a function that takes a `Value` and modifies it if it is a symbol.
pub fn modify_symbol(f: impl Fn(Symbol) -> Symbol + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_symbol(value, f.clone())
}

/// Creates a partialed modifier for keyword values.
/// Returns a function that takes a `Value` and modifies it if it is a keyword.
pub fn modify_keyword(f: impl Fn(Keyword) -> Keyword + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_keyword(value, f.clone())
}

/// Creates a partialed modifier for list values.
/// Returns a function that takes a `Value` and modifies it if it is a list.
pub fn modify_list(f: impl Fn(List) -> List + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_list(value, f.clone())
}

/// Creates a partialed modifier for vector values.
/// Returns a function that takes a `Value` and modifies it if it is a vector.
pub fn modify_vector(f: impl Fn(Vector) -> Vector + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_vector(value, f.clone())
}

/// Creates a partialed modifier for set values.
/// Returns a function that takes a `Value` and modifies it if it is a set.
pub fn modify_set(f: impl Fn(Set) -> Set + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_set(value, f.clone())
}

/// Creates a partialed modifier for map values.
/// Returns a function that takes a `Value` and modifies it if it is a map.
pub fn modify_map(f: impl Fn(Map) -> Map + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_map(value, f.clone())
}

/// Creates a partialed modifier for var values.
/// Returns a function that takes a `Value` and modifies it if it is a var.
pub fn modify_var(f: impl Fn(PtrVar) -> PtrVar + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_var(value, f.clone())
}

/// Creates a partialed modifier for function values.
/// Returns a function that takes a `Value` and modifies it if it is a function.
pub fn modify_function(
    f: impl Fn(PtrFunction) -> PtrFunction + Clone,
) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_function(value, f.clone())
}

/// Creates a partialed modifier for handle values.
/// Returns a function that takes a `Value` and modifies it if it is a handle.
pub fn modify_handle(f: impl Fn(Handle) -> Handle + Clone) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_handle(value, f.clone())
}

/// Creates a partialed modifier for metadata.
/// Returns a function that takes a `Value` and modifies its metadata.
pub fn modify_meta(
    f: impl Fn(Option<Arc<Map>>) -> Option<Arc<Map>> + Clone,
) -> impl Fn(PtrValue) -> PtrValue {
    move |value| optics::modify_meta(value, f.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partialed_modify_integer() {
        let doubler = modify_integer(|x| x * 2);
        let val = optics::review_integer(21);
        let result = doubler(val);
        assert_eq!(optics::preview_integer(&result), Some(42));
    }

    #[test]
    fn test_partialed_modify_integer_composition() {
        // Test that the partialed function can be reused
        let tripler = modify_integer(|x| x * 3);
        let val1 = optics::review_integer(10);
        let val2 = optics::review_integer(5);

        let result1 = tripler(val1);
        let result2 = tripler(val2);

        assert_eq!(optics::preview_integer(&result1), Some(30));
        assert_eq!(optics::preview_integer(&result2), Some(15));
    }

    #[test]
    fn test_partialed_modify_boolean() {
        let negater = modify_boolean(|b| !b);
        let val = optics::review_boolean(true);
        let result = negater(val);
        assert_eq!(optics::preview_boolean(&result), Some(false));
    }

    #[test]
    fn test_partialed_modify_string() {
        let uppercaser = modify_string(|s| s.to_uppercase());
        let val = optics::review_string("hello".to_string());
        let result = uppercaser(val);
        assert_eq!(optics::preview_string(&result), Some("HELLO".to_string()));
    }

    #[test]
    fn test_partialed_modify_float() {
        let halver = modify_float(|f| f / 2.0);
        let val = optics::review_float(Float::from(100.0));
        let result = halver(val);
        assert_eq!(optics::preview_float(&result), Some(Float::from(50.0)));
    }

    #[test]
    fn test_partialed_modify_integer_wrong_type() {
        let doubler = modify_integer(|x| x * 2);
        let val = optics::review_boolean(true);
        let result = doubler(val.clone());
        // Should return the original value unchanged
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_boolean_wrong_type() {
        let negater = modify_boolean(|b| !b);
        let val = optics::review_integer(42);
        let result = negater(val.clone());
        // Should return the original value unchanged
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modifiers_are_independent() {
        let times_two = modify_integer(|x| x * 2);
        let times_three = modify_integer(|x| x * 3);

        let val = optics::review_integer(10);
        let result1 = times_two(val.clone());
        let result2 = times_three(val);

        assert_eq!(optics::preview_integer(&result1), Some(20));
        assert_eq!(optics::preview_integer(&result2), Some(30));
    }

    #[test]
    fn test_partialed_modify_symbol() {
        let sym_uppercaser = modify_symbol(|s| Symbol::new_unqualified(&s.name().to_uppercase()));
        let val = optics::review_symbol(Symbol::new_unqualified("foo"));
        let result = sym_uppercaser(val);
        assert_eq!(
            optics::preview_symbol(&result).map(|s| s.name().to_string()),
            Some("FOO".to_string())
        );
    }

    #[test]
    fn test_partialed_modify_symbol_wrong_type() {
        let sym_modifier = modify_symbol(|_s| Symbol::new_unqualified("ignored"));
        let val = optics::review_integer(42);
        let result = sym_modifier(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_keyword() {
        let kw_uppercaser = modify_keyword(|k| Keyword::new_unqualified(&k.name().to_uppercase()));
        let val = optics::review_keyword(Keyword::new_unqualified("foo"));
        let result = kw_uppercaser(val);
        assert_eq!(
            optics::preview_keyword(&result).map(|k| k.name().to_string()),
            Some("FOO".to_string())
        );
    }

    #[test]
    fn test_partialed_modify_keyword_wrong_type() {
        let kw_modifier = modify_keyword(|_k| Keyword::new_unqualified("ignored"));
        let val = optics::review_integer(42);
        let result = kw_modifier(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_vector() {
        let identity_vec = modify_vector(|v| v);
        let val = optics::review_vector(Vector::new_empty());
        let result = identity_vec(val);
        assert_eq!(optics::preview_vector(&result).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_vector_wrong_type() {
        let identity_vec = modify_vector(|v| v);
        let val = optics::review_integer(42);
        let result = identity_vec(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_vector_composition() {
        let identity_vec = modify_vector(|v| v);

        let vec1 = optics::review_vector(Vector::new_empty());
        let vec2 = optics::review_vector(Vector::new_empty());

        let result1 = identity_vec(vec1);
        let result2 = identity_vec(vec2);

        assert_eq!(optics::preview_vector(&result1).unwrap().len(), 0);
        assert_eq!(optics::preview_vector(&result2).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_list() {
        let identity_list = modify_list(|l| l);
        let val = optics::review_list(List::new_empty());
        let result = identity_list(val);
        assert_eq!(optics::preview_list(&result).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_list_wrong_type() {
        let identity_list = modify_list(|l| l);
        let val = optics::review_integer(42);
        let result = identity_list(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_set() {
        let identity_set = modify_set(|s| s);
        let val = optics::review_set(Set::new_empty());
        let result = identity_set(val);
        assert_eq!(optics::preview_set(&result).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_set_wrong_type() {
        let identity_set = modify_set(|s| s);
        let val = optics::review_integer(42);
        let result = identity_set(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_map() {
        let identity_map = modify_map(|m| m);
        let val = optics::review_map(Map::new_empty());
        let result = identity_map(val);
        assert_eq!(optics::preview_map(&result).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_map_wrong_type() {
        let identity_map = modify_map(|m| m);
        let val = optics::review_integer(42);
        let result = identity_map(val.clone());
        assert_eq!(result, val);
    }

    #[test]
    fn test_partialed_modify_map_composition() {
        let identity_map = modify_map(|m| m);

        let map1 = optics::review_map(Map::new_empty());
        let map2 = optics::review_map(Map::new_empty());

        let result1 = identity_map(map1);
        let result2 = identity_map(map2);

        assert_eq!(optics::preview_map(&result1).unwrap().len(), 0);
        assert_eq!(optics::preview_map(&result2).unwrap().len(), 0);
    }

    #[test]
    fn test_partialed_modify_meta_add() {
        let add_meta = modify_meta(|_| Some(Arc::new(Map::new_empty())));

        let val = optics::review_integer(42);
        assert_eq!(optics::preview_meta(&val), None);

        let result = add_meta(val);
        assert!(optics::preview_meta(&result).is_some());
        assert_eq!(optics::preview_integer(&result), Some(42));
    }

    #[test]
    fn test_partialed_modify_meta_remove() {
        let remove_meta = modify_meta(|_| None);

        let val = optics::set_meta(optics::review_integer(42), Some(Arc::new(Map::new_empty())));
        assert!(optics::preview_meta(&val).is_some());

        let result = remove_meta(val);
        assert_eq!(optics::preview_meta(&result), None);
        assert_eq!(optics::preview_integer(&result), Some(42));
    }

    #[test]
    fn test_partialed_modify_meta_reusable() {
        let add_meta = modify_meta(|_| Some(Arc::new(Map::new_empty())));

        let val1 = optics::review_integer(42);
        let val2 = optics::review_string("test".to_string());

        let result1 = add_meta(val1);
        let result2 = add_meta(val2);

        assert!(optics::preview_meta(&result1).is_some());
        assert_eq!(optics::preview_integer(&result1), Some(42));

        assert!(optics::preview_meta(&result2).is_some());
        assert_eq!(optics::preview_string(&result2), Some("test".to_string()));
    }
}
