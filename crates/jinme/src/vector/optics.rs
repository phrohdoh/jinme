//! Optics for [`Vector`]s.
//!
//! Note: Since [`Vector`] is an immutable persistent data structure, `view_*` functions clone.

use crate::keyword::Keyword;
use crate::symbol::Symbol;
use crate::value::optics as value_optics;
use crate::value::{PtrValue, Value};
use crate::vector::Vector;

// =============================================================================
// Element access helpers for Vectors
// =============================================================================

/// View the first element of a [`Vector`], or [`None`] if the [`Value`] is not a [`Vector`] or is empty.
///
/// Returns a cloned reference to the first element (since [`Vector`] is persistent and immutable).
pub fn view_first(vector: &Vector) -> Option<PtrValue> {
    vector.get_first().map(|v| v.to_owned())
}

/// View the second element of a [`Vector`], or [`None`] if the [`Value`] is not a [`Vector`] or has fewer than 2 elements.
///
/// Returns a cloned reference to the element (since [`Vector`] is persistent and immutable).
pub fn view_second(vector: &Vector) -> Option<PtrValue> {
    vector.get_second().map(|v| v.to_owned())
}

/// View the last element of a [`Vector`], or [`None`] if the [`Value`] is not a [`Vector`] or is empty.
///
/// Returns a cloned reference to the last element (since [`Vector`] is persistent and immutable).
pub fn view_last(vector: &Vector) -> Option<PtrValue> {
    vector.get_last().map(|v| v.to_owned())
}

/// View the element at index `n` of a [`Vector`], or [`None`] if out of bounds.
///
/// Returns a cloned reference to the element (since [`Vector`] is persistent and immutable).
pub fn view_nth(vector: &Vector, n: usize) -> Option<PtrValue> {
    vector.get_nth_or_nil(n).into()
}

// =============================================================================
// Generic typed element access helpers for Vectors
// =============================================================================

/// View the first element of a [`Vector`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the first element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_first_as<T>(vector: &Vector, prism_preview: impl Fn(&Value) -> Option<T>) -> Option<T> {
    vector.get_first().as_deref().and_then(prism_preview)
}

/// View the second element of a [`Vector`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the second element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_second_as<T>(
    vector: &Vector,
    prism_preview: impl Fn(&Value) -> Option<T>,
) -> Option<T> {
    vector.get_second().as_deref().and_then(prism_preview)
}

/// View the last element of a [`Vector`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the last element exists and the preview succeeds, or [`None`] otherwise.
pub fn view_last_as<T>(vector: &Vector, prism_preview: impl Fn(&Value) -> Option<T>) -> Option<T> {
    vector.get_last().as_deref().and_then(prism_preview)
}

/// View the element at index `n` of a [`Vector`] and apply a prism preview function to it.
///
/// Takes a closure that focuses on a specific type within a Value.
/// Returns the focused value if the element at index `n` exists and the preview succeeds, or [`None`] otherwise.
pub fn view_nth_as<T>(
    vector: &Vector,
    n: usize,
    prism_preview: impl Fn(&Value) -> Option<T>,
) -> Option<T> {
    prism_preview(vector.get_nth_or_nil(n).as_ref())
}

// =============================================================================
// Concrete typed element access helpers for Vectors
// =============================================================================

/// View the first element of a [`Vector`] and extract it as a [`Symbol`], or [`None`] if not found or not a [`Symbol`].
///
/// This is a specialized helper that combines [`view_first_as`] with [`preview_symbol`](crate::value::optics::preview_symbol).
pub fn view_first_as_symbol(vector: &Vector) -> Option<Symbol> {
    view_first_as(vector, value_optics::preview_symbol)
}

/// View the first element of a [`Vector`] and extract it as a [`Keyword`], or [`None`] if not found or not a [`Keyword`].
///
/// This is a specialized helper that combines [`view_first_as`] with [`preview_keyword`](crate::value::optics::preview_keyword).
pub fn view_first_as_keyword(vector: &Vector) -> Option<Keyword> {
    view_first_as(vector, value_optics::preview_keyword)
}
