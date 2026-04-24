//! Optics for [`List`]s.
//!
//! Note: Since [`List`] is an immutable persistent data structure, `view_*` functions clone.

use crate::keyword::Keyword;
use crate::list::List;
use crate::symbol::Symbol;
use crate::value::optics as value_optics;
use crate::value::{PtrValue, Value};

// =============================================================================
// Element access helpers for Lists
// =============================================================================

/// View the first element of a [`List`], or [`None`] if the [`Value`] is not a [`List`] or is empty.
///
/// Returns a cloned reference to the first element (since [`List`] is persistent and immutable).
pub fn view_first(list: &List) -> Option<PtrValue> {
    list.get_first().map(|v| v.to_owned())
}

/// View the second element of a [`List`], or [`None`] if the [`Value`] is not a [`List`] or has fewer than 2 elements.
///
/// Returns a cloned reference to the element (since [`List`] is persistent and immutable).
pub fn view_second(list: &List) -> Option<PtrValue> {
    list.get_second().map(|v| v.to_owned())
}

/// View the last element of a [`List`], or [`None`] if the [`Value`] is not a [`List`] or is empty.
///
/// Returns a cloned reference to the last element (since [`List`] is persistent and immutable).
pub fn view_last(list: &List) -> Option<PtrValue> {
    list.get_last().map(|v| v.to_owned())
}

/// View the element at index `n` of a [`List`], or [`None`] if out of bounds.
///
/// Returns a cloned reference to the element (since [`List`] is persistent and immutable).
pub fn view_nth(list: &List, n: usize) -> Option<PtrValue> {
    list.get_nth_or_nil(n).into()
}

// =============================================================================
// Generic typed element access helpers for Lists
// =============================================================================

/// View the first element of a [`List`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the first element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_first_as<T>(list: &List, prism_preview: impl Fn(&Value) -> Option<T>) -> Option<T> {
    list.get_first().as_deref().and_then(prism_preview)
}

/// View the second element of a [`List`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the second element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_second_as<T>(list: &List, prism_preview: impl Fn(&Value) -> Option<T>) -> Option<T> {
    list.get_second().as_deref().and_then(prism_preview)
}

/// View the last element of a [`List`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the last element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_last_as<T>(list: &List, prism_preview: impl Fn(&Value) -> Option<T>) -> Option<T> {
    list.get_last().as_deref().and_then(prism_preview)
}

/// View the element at index `n` of a [`List`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the element at index `n` exists and the preview succeeds, or [`None`] otherwise.
pub fn view_nth_as<T>(
    list: &List,
    n: usize,
    prism_preview: impl Fn(&Value) -> Option<T>,
) -> Option<T> {
    prism_preview(list.get_nth_or_nil(n).as_ref())
}

// =============================================================================
// Concrete typed element access helpers for Lists
// =============================================================================

/// View the first element of a [`List`] and extract it as a [`Symbol`], or [`None`] if not found or not a [`Symbol`].
///
/// This is a specialized helper that combines [`view_first_as`] with [`preview_symbol`](crate::value::optics::preview_symbol).
pub fn view_first_as_symbol(list: &List) -> Option<Symbol> {
    view_first_as(list, value_optics::preview_symbol)
}

/// View the first element of a [`List`] and extract it as a [`Keyword`], or [`None`] if not found or not a [`Keyword`].
///
/// This is a specialized helper that combines [`view_first_as`] with [`preview_keyword`](crate::value::optics::preview_keyword).
pub fn view_first_as_keyword(list: &List) -> Option<Keyword> {
    view_first_as(list, value_optics::preview_keyword)
}
