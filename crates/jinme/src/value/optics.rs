//! This module provides optics (prisms, preview, review, modify, set) for accessing and transforming
//! different variants of `Value` using the prism2 trait-based system. Each variant type
//! (nil, boolean, integer, etc.) has a complete set of optics functions for functional, composable
//! access patterns.
//!
//! Common patterns:
//! - `prism_*()`: Returns a prism that can be used with lens combinators
//! - `preview_*()`: Extracts the inner value if it matches this type, returns `Option`
//! - `review_*()`: Constructs a `Value` from an inner value
//! - `modify_*()`: Applies a function to the inner value, returns the modified `Value` (or original if type doesn't match)
//! - `set_*()`: Sets the inner value, returns the new `Value`
//! - `try_modify_*()`: Applies a function, returns `Result` based on type match

use crate::list::optics as list_optics;
use crate::optics::prism2::{Prism, PrismImpl, PrismModify, PrismPreview, PrismReview, PrismSet, PrismTryModify};
use crate::prelude::*;
use crate::value::optics as value_optics;
use crate::vector::optics as vector_optics;
use crate::keyword::optics as keyword_optics;
use crate::symbol::optics as symbol_optics;
use ::std::sync::Arc;

// nil
// ========================================

/// Returns a prism for the `Value::Nil` variant.
pub fn prism_nil() -> impl Prism<Value, ()> {
    PrismImpl::new(
        |v| {
            if let Value::Nil(_) = v {
                Some(())
            } else {
                None
            }
        },
        |_| Value::nil_ptr(),
    )
}

/// Previews whether a `Value` is nil.
/// Returns `Some(())` if the value is nil, `None` otherwise.
pub fn preview_nil(value: &Value) -> Option<()> {
    prism_nil().preview(value)
}

/// Previews the nil value within a `Value` by reference.
/// Returns `Some(())` if the value is nil, `None` otherwise.
pub fn preview_nil_ref(value: &Value) -> Option<&()> {
    if let Value::Nil(_) = value {
        Some(&())
    } else {
        None
    }
}

/// Constructs a nil `Value`.
pub fn review_nil() -> PtrValue {
    let p = prism_nil();
    p.review(())
}

/// Applies a function to nil values. Since nil is unit, this just returns the value unchanged
/// or creates a new nil if it matched.
pub fn modify_nil(value: PtrValue, f: impl Fn(()) -> ()) -> PtrValue {
    prism_nil().modify(value, f)
}

/// Sets a nil value (no-op for the inner value, but transforms Value if it's nil).
pub fn set_nil(value: PtrValue) -> PtrValue {
    prism_nil().set(value, ())
}

/// Attempts to apply a function to nil values.
pub fn try_modify_nil(value: PtrValue, f: impl Fn(()) -> ()) -> Result<PtrValue, PtrValue> {
    prism_nil().try_modify(value, f)
}

// boolean
// ========================================

/// Returns a prism for the `Value::Boolean` variant.
pub fn prism_boolean() -> impl Prism<Value, bool> {
    PrismImpl::new(
        |v| {
            if let Value::Boolean(boolean, _) = v {
                Some(*boolean)
            } else {
                None
            }
        },
        Value::boolean_ptr,
    )
}

/// Previews whether a `Value` is a boolean and extracts the bool value.
/// Returns `Some(bool)` if the value is boolean, `None` otherwise.
pub fn preview_boolean(value: &Value) -> Option<bool> {
    prism_boolean().preview(value)
}

/// Previews the bool value within a `Value` by reference.
/// Returns `Some(&bool)` if the value is boolean, `None` otherwise.
pub fn preview_boolean_ref(value: &Value) -> Option<&bool> {
    if let Value::Boolean(boolean, _) = value {
        Some(boolean)
    } else {
        None
    }
}

/// Constructs a boolean `Value` from a bool.
pub fn review_boolean(b: bool) -> PtrValue {
    prism_boolean().review(b)
}

/// Applies a function to the inner bool of a `Value`, if it is a boolean.
/// Returns the modified `Value` if this is a boolean, otherwise returns the original unchanged.
pub fn modify_boolean(value: PtrValue, f: impl Fn(bool) -> bool) -> PtrValue {
    prism_boolean().modify(value, f)
}

/// Sets the bool value of a `Value`, if it is a boolean.
/// Returns a new `Value` with the updated bool if this is a boolean, otherwise returns the original unchanged.
pub fn set_boolean(value: PtrValue, b: bool) -> PtrValue {
    prism_boolean().set(value, b)
}

/// Attempts to apply a function to the inner bool of a `Value`.
/// Returns `Ok(modified_value)` if this is a boolean, `Err(original_value)` otherwise.
pub fn try_modify_boolean(value: PtrValue, f: impl Fn(bool) -> bool) -> Result<PtrValue, PtrValue> {
    prism_boolean().try_modify(value, f)
}

// integer
// ========================================

/// Returns a prism for the `Value::Integer` variant.
pub fn prism_integer() -> impl Prism<Value, i64> {
    PrismImpl::new(
        |v| {
            if let Value::Integer(i, _) = v {
                Some(*i)
            } else {
                None
            }
        },
        Value::integer_ptr,
    )
}

/// Previews whether a `Value` is an integer and extracts the i64 value.
/// Returns `Some(i64)` if the value is an integer, `None` otherwise.
pub fn preview_integer(value: &Value) -> Option<i64> {
    prism_integer().preview(value)
}

/// Previews the i64 value within a `Value` by reference.
/// Returns `Some(i64)` if the value is an integer, `None` otherwise.
pub fn preview_integer_ref(value: &Value) -> Option<&i64> {
    if let Value::Integer(i, _) = value {
        Some(i)
    } else {
        None
    }
}

/// Constructs an integer `Value` from an i64.
pub fn review_integer(i: i64) -> PtrValue {
    prism_integer().review(i)
}

/// Applies a function to the inner i64 of a `Value`, if it is an integer.
/// Returns the modified `Value` if this is an integer, otherwise returns the original unchanged.
pub fn modify_integer(value: PtrValue, f: impl Fn(i64) -> i64) -> PtrValue {
    prism_integer().modify(value, f)
}

/// Sets the i64 value of a `Value`, if it is an integer.
/// Returns a new `Value` with the updated i64 if this is an integer, otherwise returns the original unchanged.
pub fn set_integer(value: PtrValue, i: i64) -> PtrValue {
    prism_integer().set(value, i)
}

/// Attempts to apply a function to the inner i64 of a `Value`.
/// Returns `Ok(modified_value)` if this is an integer, `Err(original_value)` otherwise.
pub fn try_modify_integer(value: PtrValue, f: impl Fn(i64) -> i64) -> Result<PtrValue, PtrValue> {
    prism_integer().try_modify(value, f)
}

// float
// ========================================

/// Returns a prism for the `Value::Float` variant.
pub fn prism_float() -> impl Prism<Value, Float> {
    // prism2::prism_f64()
    PrismImpl::new(
        |v| {
            if let Value::Float(f, _) = v {
                Some(f.clone())
            } else {
                None
            }
        },
        Value::float_ptr,
    )
}

/// Previews whether a `Value` is a float and extracts the f64 value.
/// Returns `Some(f64)` if the value is a float, `None` otherwise.
pub fn preview_float(value: &Value) -> Option<Float> {
    prism_float().preview(value)
}

/// Previews the `Float` value within a `Value` by reference.
/// Returns `Some(&Float)` if the value is a float, `None` otherwise.
pub fn preview_float_ref(value: &Value) -> Option<&Float> {
    if let Value::Float(f, _) = value {
        Some(f)
    } else {
        None
    }
}

/// Constructs a float `Value` from a `Float`.
pub fn review_float(float: Float) -> PtrValue {
    prism_float().review(float)
}

/// Applies a function to the inner `Float` of a `Value`, if it is a float.
/// Returns the modified `Value` if this is a float, otherwise returns the original unchanged.
pub fn modify_float(value: PtrValue, f: impl Fn(Float) -> Float) -> PtrValue {
    prism_float().modify(value, f)
}

/// Sets the `Float` value of a `Value`, if it is a float.
/// Returns a new `Value` with the updated `Float` if this is a float, otherwise returns the original unchanged.
pub fn set_float(value: PtrValue, float: Float) -> PtrValue {
    prism_float().set(value, float)
}

/// Attempts to apply a function to the inner `Float` of a `Value`.
/// Returns `Ok(modified_value)` if this is a float, `Err(original_value)` otherwise.
pub fn try_modify_float(value: PtrValue, f: impl Fn(Float) -> Float) -> Result<PtrValue, PtrValue> {
    prism_float().try_modify(value, f)
}

// string
// ========================================

/// Returns a prism for the `Value::String` variant.
pub fn prism_string() -> impl Prism<Value, String> {
    PrismImpl::new(
        |v| {
            if let Value::String(s, _) = v {
                Some(s.clone())
            } else {
                None
            }
        },
        Value::string_ptr,
    )
}

/// Previews whether a `Value` is a string and extracts the String value.
/// Returns `Some(String)` if the value is a string, `None` otherwise.
pub fn preview_string(value: &Value) -> Option<String> {
    prism_string().preview(value)
}

/// Previews the `String` value within a `Value` by reference.
/// Returns `Some(&str)` if the value is a string, `None` otherwise.
/// This avoids cloning the `String` value.
pub fn preview_string_ref(value: &Value) -> Option<&str> {
    if let Value::String(s, _) = value {
        Some(s.as_str())
    } else {
        None
    }
}

/// Constructs a string `Value` from a String.
pub fn review_string(s: String) -> PtrValue {
    prism_string().review(s)
}

/// Applies a function to the inner String of a `Value`, if it is a string.
/// Returns the modified `Value` if this is a string, otherwise returns the original unchanged.
pub fn modify_string(value: PtrValue, f: impl Fn(String) -> String) -> PtrValue {
    prism_string().modify(value, f)
}

/// Sets the String value of a `Value`, if it is a string.
/// Returns a new `Value` with the updated String if this is a string, otherwise returns the original unchanged.
pub fn set_string(value: PtrValue, s: String) -> PtrValue {
    prism_string().set(value, s)
}

/// Attempts to apply a function to the inner String of a `Value`.
/// Returns `Ok(modified_value)` if this is a string, `Err(original_value)` otherwise.
pub fn try_modify_string(
    value: PtrValue,
    f: impl Fn(String) -> String,
) -> Result<PtrValue, PtrValue> {
    prism_string().try_modify(value, f)
}

// symbol
// ========================================

/// Returns a prism for the `Value::Symbol` variant.
pub fn prism_symbol() -> impl Prism<Value, Symbol> {
    PrismImpl::new(
        |v| {
            if let Value::Symbol(sym, _) = v {
                Some(sym.clone())
            } else {
                None
            }
        },
        Value::symbol_ptr,
    )
}

/// Previews whether a `Value` is a symbol and extracts the Symbol value.
/// Returns `Some(Symbol)` if the value is a symbol, `None` otherwise.
pub fn preview_symbol(value: &Value) -> Option<Symbol> {
    prism_symbol().preview(value)
}

/// Previews whether a `Value` is an unqualified symbol and extracts the `SymbolUnqualified` value.
/// Returns `Some(SymbolUnqualified)` if the value is an unqualified symbol, `None` otherwise.
pub fn preview_symbol_unqualified(value: &Value) -> Option<SymbolUnqualified> {
    preview_symbol(value).and_then(symbol_optics::preview_unqualified)
}

/// Previews whether a `Value` is a qualified symbol and extracts the `SymbolQualified` value.
/// Returns `Some(SymbolQualified)` if the value is a qualified symbol, `None` otherwise.
pub fn preview_symbol_qualified(value: &Value) -> Option<SymbolQualified> {
    preview_symbol(value).and_then(symbol_optics::preview_qualified)
}

/// Previews the `Symbol` value within a `Value` by reference.
/// Returns `Some(&Symbol)` if the value is a symbol, `None` otherwise.
/// This avoids cloning the `Symbol` value.
pub fn preview_symbol_ref(value: &Value) -> Option<&Symbol> {
    if let Value::Symbol(sym, _) = value {
        Some(sym)
    } else {
        None
    }
}

/// Constructs a symbol `Value` from a Symbol.
pub fn review_symbol(sym: Symbol) -> PtrValue {
    prism_symbol().review(sym)
}

/// Applies a function to the inner Symbol of a `Value`, if it is a symbol.
/// Returns the modified `Value` if this is a symbol, otherwise returns the original unchanged.
pub fn modify_symbol(value: PtrValue, f: impl Fn(Symbol) -> Symbol) -> PtrValue {
    prism_symbol().modify(value, f)
}

/// Sets the Symbol value of a `Value`, if it is a symbol.
/// Returns a new `Value` with the updated Symbol if this is a symbol, otherwise returns the original unchanged.
pub fn set_symbol(value: PtrValue, sym: Symbol) -> PtrValue {
    prism_symbol().set(value, sym)
}

/// Attempts to apply a function to the inner Symbol of a `Value`.
/// Returns `Ok(modified_value)` if this is a symbol, `Err(original_value)` otherwise.
pub fn try_modify_symbol(
    value: PtrValue,
    f: impl Fn(Symbol) -> Symbol,
) -> Result<PtrValue, PtrValue> {
    prism_symbol().try_modify(value, f)
}

// keyword
// ========================================

/// Returns a prism for the `Value::Keyword` variant.
pub fn prism_keyword() -> impl Prism<Value, Keyword> {
    PrismImpl::new(
        |v| {
            if let Value::Keyword(kw, _) = v {
                Some(kw.clone())
            } else {
                None
            }
        },
        Value::keyword_ptr,
    )
}

/// Previews whether a `Value` is a keyword and extracts the Keyword value.
/// Returns `Some(Keyword)` if the value is a keyword, `None` otherwise.
pub fn preview_keyword(value: &Value) -> Option<Keyword> {
    prism_keyword().preview(value)
}

/// Previews whether a `Value` is an unqualified keyword and extracts the `KeywordUnqualified` value.
/// Returns `Some(KeywordUnqualified)` if the value is an unqualified keyword, `None` otherwise.
pub fn preview_keyword_unqualified(value: &Value) -> Option<KeywordUnqualified> {
    preview_keyword(value).and_then(keyword_optics::preview_unqualified)
}

/// Previews whether a `Value` is a qualified keyword and extracts the `KeywordQualified` value.
/// Returns `Some(KeywordQualified)` if the value is a qualified keyword, `None` otherwise.
pub fn preview_keyword_qualified(value: &Value) -> Option<KeywordQualified> {
    preview_keyword(value).and_then(keyword_optics::preview_qualified)
}

/// Previews the `Keyword` value within a `Value` by reference.
/// Returns `Some(&Keyword)` if the value is a keyword, `None` otherwise.
/// This avoids cloning the `Keyword` value.
pub fn preview_keyword_ref(value: &Value) -> Option<&Keyword> {
    if let Value::Keyword(kw, _) = value {
        Some(kw)
    } else {
        None
    }
}

/// Constructs a keyword `Value` from a Keyword.
pub fn review_keyword(kw: Keyword) -> PtrValue {
    prism_keyword().review(kw)
}

/// Applies a function to the inner Keyword of a `Value`, if it is a keyword.
/// Returns the modified `Value` if this is a keyword, otherwise returns the original unchanged.
pub fn modify_keyword(value: PtrValue, f: impl Fn(Keyword) -> Keyword) -> PtrValue {
    prism_keyword().modify(value, f)
}

/// Sets the Keyword value of a `Value`, if it is a keyword.
/// Returns a new `Value` with the updated Keyword if this is a keyword, otherwise returns the original unchanged.
pub fn set_keyword(value: PtrValue, kw: Keyword) -> PtrValue {
    prism_keyword().set(value, kw)
}

/// Attempts to apply a function to the inner Keyword of a `Value`.
/// Returns `Ok(modified_value)` if this is a keyword, `Err(original_value)` otherwise.
pub fn try_modify_keyword(
    value: PtrValue,
    f: impl Fn(Keyword) -> Keyword,
) -> Result<PtrValue, PtrValue> {
    prism_keyword().try_modify(value, f)
}

// list
// ========================================

/// Returns a prism for the `Value::List` variant.
pub fn prism_list() -> impl Prism<Value, List> {
    PrismImpl::new(
        |v| {
            if let Value::List(list, _) = v {
                Some(list.clone())
            } else {
                None
            }
        },
        Value::list_ptr,
    )
}

/// Previews whether a `Value` is a `List` and extracts the `List` value.
/// Returns `Some(List)` if the value is a `List`, `None` otherwise.
pub fn preview_list(value: &Value) -> Option<List> {
    prism_list().preview(value)
}

/// Previews the first element of a `Value` if it is a `List`.
/// Returns `Some(first_element)` if the value is a `List` with at least one element, `None` otherwise.
pub fn preview_list_first(value: &Value) -> Option<PtrValue> {
    value_optics::preview_list(value)
        .as_ref()
        .and_then(list_optics::view_first)
}

/// Previews the second element of a `Value` if it is a `List`.
/// Returns `Some(second_element)` if the value is a `List` with at least two elements, `None` otherwise.
pub fn preview_list_second(value: &Value) -> Option<PtrValue> {
    value_optics::preview_list(value)
        .as_ref()
        .and_then(list_optics::view_second)
}

/// Previews the last element of a `Value` if it is a `List`.
/// Returns `Some(last_element)` if the value is a `List` with at least one element, `None` otherwise.
pub fn preview_list_last(value: &Value) -> Option<PtrValue> {
    value_optics::preview_list(value)
        .as_ref()
        .and_then(list_optics::view_last)
}

/// Previews the nth element of a `Value` if it is a `List`.
///
/// # Arguments
///
/// * `value` - The value to preview
/// * `n` - The 0-based index of the element to retrieve
///
/// Returns `Some(nth_element)` if the value is a `List` with at least `n+1` elements, `None` otherwise.
pub fn preview_list_nth(value: &Value, n: usize) -> Option<PtrValue> {
    value_optics::preview_list(value)
        .as_ref()
        .and_then(|list| list_optics::view_nth(list, n))
}

/// Previews the `List` value within a `Value` by reference.
/// Returns `Some(&List)` if the value is a `List`, `None` otherwise.
/// This avoids cloning the `List` value.
pub fn preview_list_ref(value: &Value) -> Option<&List> {
    if let Value::List(list, _) = value {
        Some(list)
    } else {
        None
    }
}

/// Constructs a list `Value` from a List.
pub fn review_list(list: List) -> PtrValue {
    prism_list().review(list)
}

/// Applies a function to the inner List of a `Value`, if it is a list.
/// Returns the modified `Value` if this is a list, otherwise returns the original unchanged.
pub fn modify_list(value: PtrValue, f: impl Fn(List) -> List) -> PtrValue {
    prism_list().modify(value, f)
}

/// Sets the List value of a `Value`, if it is a list.
/// Returns a new `Value` with the updated List if this is a list, otherwise returns the original unchanged.
pub fn set_list(value: PtrValue, list: List) -> PtrValue {
    prism_list().set(value, list)
}

/// Attempts to apply a function to the inner List of a `Value`.
/// Returns `Ok(modified_value)` if this is a list, `Err(original_value)` otherwise.
pub fn try_modify_list(value: PtrValue, f: impl Fn(List) -> List) -> Result<PtrValue, PtrValue> {
    prism_list().try_modify(value, f)
}

// vector
// ========================================

/// Returns a prism for the `Value::Vector` variant.
pub fn prism_vector() -> impl Prism<Value, Vector> {
    PrismImpl::new(
        |v| {
            if let Value::Vector(vec, _) = v {
                Some(vec.clone())
            } else {
                None
            }
        },
        Value::vector_ptr,
    )
}

/// Previews whether a `Value` is a vector and extracts the Vector value.
/// Returns `Some(Vector)` if the value is a vector, `None` otherwise.
pub fn preview_vector(value: &Value) -> Option<Vector> {
    prism_vector().preview(value)
}

/// Previews the first element of a `Value` if it is a vector.
/// Returns `Some(first_element)` if the value is a vector with at least one element, `None` otherwise.
pub fn preview_vector_first(value: &Value) -> Option<PtrValue> {
    value_optics::preview_vector(value)
        .as_ref()
        .and_then(vector_optics::view_first)
}

/// Previews the second element of a `Value` if it is a vector.
/// Returns `Some(second_element)` if the value is a vector with at least two elements, `None` otherwise.
pub fn preview_vector_second(value: &Value) -> Option<PtrValue> {
    value_optics::preview_vector(value)
        .as_ref()
        .and_then(vector_optics::view_second)
}

/// Previews the last element of a `Value` if it is a vector.
/// Returns `Some(last_element)` if the value is a vector with at least one element, `None` otherwise.
pub fn preview_vector_last(value: &Value) -> Option<PtrValue> {
    value_optics::preview_vector(value)
        .as_ref()
        .and_then(vector_optics::view_last)
}

/// Previews the nth element of a `Value` if it is a vector.
///
/// # Arguments
///
/// * `value` - The value to preview
/// * `n` - The 0-based index of the element to retrieve
///
/// Returns `Some(nth_element)` if the value is a vector with at least `n+1` elements, `None` otherwise.
pub fn preview_vector_nth(value: &Value, n: usize) -> Option<PtrValue> {
    value_optics::preview_vector(value)
        .as_ref()
        .and_then(|vector| vector_optics::view_nth(vector, n))
}

/// Previews the `Vector` value within a `Value` by reference.
/// Returns `Some(&Vector)` if the value is a vector, `None` otherwise.
/// This avoids cloning the `Vector` value.
pub fn preview_vector_ref(value: &Value) -> Option<&Vector> {
    if let Value::Vector(vec, _) = value {
        Some(vec)
    } else {
        None
    }
}

/// Constructs a vector `Value` from a Vector.
pub fn review_vector(vec: Vector) -> PtrValue {
    prism_vector().review(vec)
}

/// Applies a function to the inner Vector of a `Value`, if it is a vector.
/// Returns the modified `Value` if this is a vector, otherwise returns the original unchanged.
pub fn modify_vector(value: PtrValue, f: impl Fn(Vector) -> Vector) -> PtrValue {
    prism_vector().modify(value, f)
}

/// Sets the Vector value of a `Value`, if it is a vector.
/// Returns a new `Value` with the updated Vector if this is a vector, otherwise returns the original unchanged.
pub fn set_vector(value: PtrValue, vec: Vector) -> PtrValue {
    prism_vector().set(value, vec)
}

/// Attempts to apply a function to the inner Vector of a `Value`.
/// Returns `Ok(modified_value)` if this is a vector, `Err(original_value)` otherwise.
pub fn try_modify_vector(
    value: PtrValue,
    f: impl Fn(Vector) -> Vector,
) -> Result<PtrValue, PtrValue> {
    prism_vector().try_modify(value, f)
}

// set
// ========================================

/// Returns a prism for the `Value::Set` variant.
pub fn prism_set() -> impl Prism<Value, Set> {
    PrismImpl::new(
        |v| {
            if let Value::Set(set, _) = v {
                Some(set.clone())
            } else {
                None
            }
        },
        Value::set_ptr,
    )
}

/// Previews whether a `Value` is a set and extracts the Set value.
/// Returns `Some(Set)` if the value is a set, `None` otherwise.
pub fn preview_set(value: &Value) -> Option<Set> {
    prism_set().preview(value)
}

/// Previews the `Set` value within a `Value` by reference.
/// Returns `Some(&Set)` if the value is a set, `None` otherwise.
/// This avoids cloning the `Set` value.
pub fn preview_set_ref(value: &Value) -> Option<&Set> {
    if let Value::Set(set, _) = value {
        Some(set)
    } else {
        None
    }
}

/// Constructs a set `Value` from a Set.
pub fn review_set(set: Set) -> PtrValue {
    prism_set().review(set)
}

/// Applies a function to the inner Set of a `Value`, if it is a set.
/// Returns the modified `Value` if this is a set, otherwise returns the original unchanged.
pub fn modify_set(value: PtrValue, f: impl Fn(Set) -> Set) -> PtrValue {
    prism_set().modify(value, f)
}

/// Sets the Set value of a `Value`, if it is a set.
/// Returns a new `Value` with the updated Set if this is a set, otherwise returns the original unchanged.
pub fn set_set(value: PtrValue, set: Set) -> PtrValue {
    prism_set().set(value, set)
}

/// Attempts to apply a function to the inner Set of a `Value`.
/// Returns `Ok(modified_value)` if this is a set, `Err(original_value)` otherwise.
pub fn try_modify_set(value: PtrValue, f: impl Fn(Set) -> Set) -> Result<PtrValue, PtrValue> {
    prism_set().try_modify(value, f)
}

// map
// ========================================

/// Returns a prism for the `Value::Map` variant.
pub fn prism_map() -> impl Prism<Value, Map> {
    PrismImpl::new(
        |v| {
            if let Value::Map(map, _) = v {
                Some(map.clone())
            } else {
                None
            }
        },
        Value::map_ptr,
    )
}

/// Previews whether a `Value` is a map and extracts the Map value.
/// Returns `Some(Map)` if the value is a map, `None` otherwise.
pub fn preview_map(value: &Value) -> Option<Map> {
    prism_map().preview(value)
}

/// Previews the `Map` value within a `Value` by reference.
/// Returns `Some(&Map)` if the value is a map, `None` otherwise.
/// This avoids cloning the `Map` value.
pub fn preview_map_ref(value: &Value) -> Option<&Map> {
    if let Value::Map(map, _) = value {
        Some(map)
    } else {
        None
    }
}

pub fn view_map(value: &Value) -> Option<&Map> {
    preview_map_ref(value)
}

/// Constructs a map `Value` from a Map.
pub fn review_map(map: Map) -> PtrValue {
    prism_map().review(map)
}

/// Applies a function to the inner Map of a `Value`, if it is a map.
/// Returns the modified `Value` if this is a map, otherwise returns the original unchanged.
pub fn modify_map(value: PtrValue, f: impl Fn(Map) -> Map) -> PtrValue {
    prism_map().modify(value, f)
}

/// Sets the Map value of a `Value`, if it is a map.
/// Returns a new `Value` with the updated Map if this is a map, otherwise returns the original unchanged.
pub fn set_map(value: PtrValue, map: Map) -> PtrValue {
    prism_map().set(value, map)
}

/// Attempts to apply a function to the inner Map of a `Value`.
/// Returns `Ok(modified_value)` if this is a map, `Err(original_value)` otherwise.
pub fn try_modify_map(value: PtrValue, f: impl Fn(Map) -> Map) -> Result<PtrValue, PtrValue> {
    prism_map().try_modify(value, f)
}

// var
// ========================================

/// Returns a prism for the `Value::Var` variant.
pub fn prism_var() -> impl Prism<Value, PtrVar> {
    PrismImpl::new(
        |v| {
            if let Value::Var(var, _) = v {
                Some(var.clone())
            } else {
                None
            }
        },
        Value::var_ptr,
    )
}

/// Previews whether a `Value` is a var and extracts the PtrVar value.
/// Returns `Some(PtrVar)` if the value is a var, `None` otherwise.
pub fn preview_var(value: &Value) -> Option<PtrVar> {
    prism_var().preview(value)
}

/// Previews the `PtrVar` value within a `Value` by reference.
/// Returns `Some(&PtrVar)` if the value is a var, `None` otherwise.
/// This avoids cloning the `PtrVar` value.
pub fn preview_var_ref(value: &Value) -> Option<&PtrVar> {
    if let Value::Var(var, _) = value {
        Some(var)
    } else {
        None
    }
}

/// Constructs a var `Value` from an PtrVar.
pub fn review_var(var: PtrVar) -> PtrValue {
    prism_var().review(var)
}

/// Applies a function to the inner PtrVar of a `Value`, if it is a var.
/// Returns the modified `Value` if this is a var, otherwise returns the original unchanged.
pub fn modify_var(value: PtrValue, f: impl Fn(PtrVar) -> PtrVar) -> PtrValue {
    prism_var().modify(value, f)
}

/// Sets the PtrVar value of a `Value`, if it is a var.
/// Returns a new `Value` with the updated PtrVar if this is a var, otherwise returns the original unchanged.
pub fn set_var(value: PtrValue, var: PtrVar) -> PtrValue {
    prism_var().set(value, var)
}

/// Attempts to apply a function to the inner PtrVar of a `Value`.
/// Returns `Ok(modified_value)` if this is a var, `Err(original_value)` otherwise.
pub fn try_modify_var(value: PtrValue, f: impl Fn(PtrVar) -> PtrVar) -> Result<PtrValue, PtrValue> {
    prism_var().try_modify(value, f)
}

// function
// ========================================

/// Returns a prism for the `Value::Function` variant.
pub fn prism_function() -> impl Prism<Value, PtrFunction> {
    PrismImpl::new(
        |v| {
            if let Value::Function(func, _) = v {
                Some(func.clone())
            } else {
                None
            }
        },
        Value::function_ptr,
    )
}

/// Previews whether a `Value` is a function and extracts the PtrFunction value.
/// Returns `Some(PtrFunction)` if the value is a function, `None` otherwise.
pub fn preview_function(value: &Value) -> Option<PtrFunction> {
    prism_function().preview(value)
}

/// Previews the `PtrFunction` value within a `Value` by reference.
/// Returns `Some(&PtrFunction)` if the value is a function, `None` otherwise.
/// This avoids cloning the `PtrFunction` value.
pub fn preview_function_ref(value: &Value) -> Option<&PtrFunction> {
    if let Value::Function(func, _) = value {
        Some(func)
    } else {
        None
    }
}

/// Constructs a function `Value` from an PtrFunction.
pub fn review_function(func: PtrFunction) -> PtrValue {
    prism_function().review(func)
}

/// Applies a function to the inner PtrFunction of a `Value`, if it is a function.
/// Returns the modified `Value` if this is a function, otherwise returns the original unchanged.
pub fn modify_function(value: PtrValue, f: impl Fn(PtrFunction) -> PtrFunction) -> PtrValue {
    prism_function().modify(value, f)
}

/// Sets the PtrFunction value of a `Value`, if it is a function.
/// Returns a new `Value` with the updated PtrFunction if this is a function, otherwise returns the original unchanged.
pub fn set_function(value: PtrValue, func: PtrFunction) -> PtrValue {
    prism_function().set(value, func)
}

/// Attempts to apply a function to the inner PtrFunction of a `Value`.
/// Returns `Ok(modified_value)` if this is a function, `Err(original_value)` otherwise.
pub fn try_modify_function(
    value: PtrValue,
    f: impl Fn(PtrFunction) -> PtrFunction,
) -> Result<PtrValue, PtrValue> {
    prism_function().try_modify(value, f)
}

// handle
// ========================================

/// Returns a prism for the `Value::Handle` variant.
pub fn prism_handle() -> impl Prism<Value, Handle> {
    PrismImpl::new(
        |v| {
            if let Value::Handle(handle, _) = v {
                Some(handle.clone())
            } else {
                None
            }
        },
        Value::handle_ptr,
    )
}

/// Previews whether a `Value` is a handle and extracts the Handle value.
/// Returns `Some(Handle)` if the value is a handle, `None` otherwise.
pub fn preview_handle(value: &Value) -> Option<Handle> {
    prism_handle().preview(value)
}

/// Previews the `Handle` value within a `Value` by reference.
/// Returns `Some(&Handle)` if the value is a handle, `None` otherwise.
/// This avoids cloning the `Handle` value.
pub fn preview_handle_ref(value: &Value) -> Option<&Handle> {
    if let Value::Handle(handle, _) = value {
        Some(handle)
    } else {
        None
    }
}

/// Constructs a handle `Value` from a Handle.
pub fn review_handle(handle: Handle) -> PtrValue {
    prism_handle().review(handle)
}

/// Applies a function to the inner Handle of a `Value`, if it is a handle.
/// Returns the modified `Value` if this is a handle, otherwise returns the original unchanged.
pub fn modify_handle(value: PtrValue, f: impl Fn(Handle) -> Handle) -> PtrValue {
    prism_handle().modify(value, f)
}

/// Sets the Handle value of a `Value`, if it is a handle.
/// Returns a new `Value` with the updated Handle if this is a handle, otherwise returns the original unchanged.
pub fn set_handle(value: PtrValue, handle: Handle) -> PtrValue {
    prism_handle().set(value, handle)
}

/// Attempts to apply a function to the inner Handle of a `Value`.
/// Returns `Ok(modified_value)` if this is a handle, `Err(original_value)` otherwise.
pub fn try_modify_handle(
    value: PtrValue,
    f: impl Fn(Handle) -> Handle,
) -> Result<PtrValue, PtrValue> {
    prism_handle().try_modify(value, f)
}

// meta (metadata)
// ========================================

/// Previews the metadata of a `Value`.
/// Returns `Some(Arc<Map>)` if metadata is present, `None` otherwise.
/// This works on any `Value` variant since all variants have embedded metadata.
pub fn preview_meta(value: &Value) -> Option<Arc<Map>> {
    match value {
        Value::Nil(meta) => meta.clone(),
        Value::Boolean(_, meta) => meta.clone(),
        Value::Integer(_, meta) => meta.clone(),
        Value::Float(_, meta) => meta.clone(),
        Value::String(_, meta) => meta.clone(),
        Value::Symbol(_, meta) => meta.clone(),
        Value::Keyword(_, meta) => meta.clone(),
        Value::List(_, meta) => meta.clone(),
        Value::Vector(_, meta) => meta.clone(),
        Value::Set(_, meta) => meta.clone(),
        Value::Map(_, meta) => meta.clone(),
        Value::Var(_, meta) => meta.clone(),
        Value::Function(_, meta) => meta.clone(),
        Value::Handle(_, meta) => meta.clone(),
    }
}

/// Previews the metadata of a `Value` by reference.
/// Returns `&Option<Arc<Map>>` - a reference to the metadata, which will be `&Some(rc_map)` if
/// metadata is present or `&None` if no metadata. The reference is valid as long as the `Value` is valid.
/// This avoids cloning the metadata map. This works on any `Value` variant since all have embedded metadata.
pub fn preview_meta_ref(value: &Value) -> Option<&Arc<Map>> {
    // This is a special case - metadata is accessed directly from the enum variant
    // since it's part of the Value structure for each variant
    match value {
        Value::Nil(meta) => meta.as_ref(),
        Value::Boolean(_, meta) => meta.as_ref(),
        Value::Integer(_, meta) => meta.as_ref(),
        Value::Float(_, meta) => meta.as_ref(),
        Value::String(_, meta) => meta.as_ref(),
        Value::Symbol(_, meta) => meta.as_ref(),
        Value::Keyword(_, meta) => meta.as_ref(),
        Value::List(_, meta) => meta.as_ref(),
        Value::Vector(_, meta) => meta.as_ref(),
        Value::Set(_, meta) => meta.as_ref(),
        Value::Map(_, meta) => meta.as_ref(),
        Value::Var(_, meta) => meta.as_ref(),
        Value::Function(_, meta) => meta.as_ref(),
        Value::Handle(_, meta) => meta.as_ref(),
    }
}

/// Applies a function to the metadata of a `Value`.
/// Returns a new `Value` with the modified metadata. This works on any variant.
pub fn modify_meta(value: PtrValue, f: impl Fn(Option<Arc<Map>>) -> Option<Arc<Map>>) -> PtrValue {
    match (*value).clone() {
        Value::Nil(meta) => Arc::new(Value::Nil(f(meta))),
        Value::Boolean(b, meta) => Arc::new(Value::Boolean(b, f(meta))),
        Value::Integer(i, meta) => Arc::new(Value::Integer(i, f(meta))),
        Value::Float(fl, meta) => Arc::new(Value::Float(fl, f(meta))),
        Value::String(s, meta) => Arc::new(Value::String(s, f(meta))),
        Value::Symbol(sym, meta) => Arc::new(Value::Symbol(sym, f(meta))),
        Value::Keyword(kw, meta) => Arc::new(Value::Keyword(kw, f(meta))),
        Value::List(list, meta) => Arc::new(Value::List(list, f(meta))),
        Value::Vector(vec, meta) => Arc::new(Value::Vector(vec, f(meta))),
        Value::Set(set, meta) => Arc::new(Value::Set(set, f(meta))),
        Value::Map(map, meta) => Arc::new(Value::Map(map, f(meta))),
        Value::Var(var, meta) => Arc::new(Value::Var(var, f(meta))),
        Value::Function(func, meta) => Arc::new(Value::Function(func, f(meta))),
        Value::Handle(handle, meta) => Arc::new(Value::Handle(handle, f(meta))),
    }
}

/// Sets the metadata of a `Value`.
/// Returns a new `Value` with the updated metadata, replacing any existing metadata.
pub fn set_meta(value: PtrValue, meta: Option<Arc<Map>>) -> PtrValue {
    modify_meta(value, |_| meta.clone())
}

/// Attempts to get the metadata of a `Value` if it exists.
/// Returns `Ok(meta_map)` if metadata is present, `Err(original_value)` if metadata is None.
pub fn try_get_meta(value: PtrValue) -> Result<Arc<Map>, PtrValue> {
    if let Some(meta) = preview_meta(&value) {
        Ok(meta)
    } else {
        Err(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic type tests
    #[test]
    fn test_modify_integer() {
        let val = review_integer(42);
        let modified = modify_integer(val, |i| i * 2);
        assert_eq!(preview_integer(&modified), Some(84));
    }

    #[test]
    fn test_modify_integer_wrong_type() {
        let val = review_boolean(true);
        let modified = modify_integer(val.clone(), |i| i * 2);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_preview_nil() {
        let val = review_nil();
        assert_eq!(preview_nil(&val), Some(()));
    }

    #[test]
    fn test_preview_meta_none() {
        let val = review_integer(42);
        assert_eq!(preview_meta(&val), None);
    }

    #[test]
    fn test_preview_meta_with_metadata() {
        let meta = Map::new_empty();
        let val = set_meta(review_integer(42), Some(Arc::new(meta)));
        assert!(preview_meta(&val).is_some());
    }

    #[test]
    fn test_modify_meta_add() {
        let val = review_integer(42);
        let modified = modify_meta(val, |_| Some(Arc::new(Map::new_empty())));
        assert!(preview_meta(&modified).is_some());
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_modify_meta_remove() {
        let val = set_meta(review_integer(42), Some(Arc::new(Map::new_empty())));
        assert!(preview_meta(&val).is_some());
        let modified = modify_meta(val, |_| None);
        assert_eq!(preview_meta(&modified), None);
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_set_meta_replaces() {
        let val = set_meta(review_integer(42), Some(Arc::new(Map::new_empty())));
        assert!(preview_meta(&val).is_some());
        let new_meta = Map::new_empty();
        let modified = set_meta(val, Some(Arc::new(new_meta)));
        assert!(preview_meta(&modified).is_some());
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_set_meta_to_none() {
        let val = set_meta(review_integer(42), Some(Arc::new(Map::new_empty())));
        let modified = set_meta(val, None);
        assert_eq!(preview_meta(&modified), None);
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_try_get_meta_success() {
        let meta = Map::new_empty();
        let val = set_meta(review_integer(42), Some(Arc::new(meta)));
        let result = try_get_meta(val);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_get_meta_failure() {
        let val = review_integer(42);
        let result = try_get_meta(val.clone());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), val);
    }

    #[test]
    fn test_meta_on_various_types() {
        let values = vec![
            review_nil(),
            review_boolean(true),
            review_integer(42),
            review_float(Float::from(3.14)),
            review_string("test".to_string()),
            review_symbol(Symbol::new_unqualified("sym")),
            review_keyword(Keyword::new_unqualified("kw")),
            review_list(List::new_empty()),
            review_vector(Vector::new_empty()),
            review_set(Set::new_empty()),
            review_map(Map::new_empty()),
        ];

        for val in values {
            assert_eq!(preview_meta(&val), None);
            let with_meta = set_meta(val.clone(), Some(Arc::new(Map::new_empty())));
            assert!(preview_meta(&with_meta).is_some());
            let without_meta = set_meta(with_meta, None);
            assert_eq!(preview_meta(&without_meta), None);
        }
    }

    #[test]
    fn test_preview_meta_ref() {
        let val_no_meta = review_integer(42);
        assert!(preview_meta_ref(&val_no_meta).is_none());
        let val_with_meta = set_meta(review_integer(42), Some(Arc::new(Map::new_empty())));
        let meta_ref = preview_meta_ref(&val_with_meta);
        assert!(meta_ref.is_some());
    }

    #[test]
    fn test_preview_nil_ref() {
        assert_eq!(preview_nil_ref(&review_nil()), Some(&()));
        assert_eq!(preview_nil_ref(&review_boolean(true)), None);
    }

    #[test]
    fn test_preview_boolean_ref() {
        assert_eq!(preview_boolean_ref(&review_boolean(true)), Some(&true));
        assert_eq!(preview_boolean_ref(&review_boolean(false)), Some(&false));
        assert_eq!(preview_boolean_ref(&review_nil()), None);
    }
}
