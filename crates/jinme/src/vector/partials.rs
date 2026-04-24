use crate::value::{PtrValue, Value};
use crate::vector::Vector;

/// View the last element of a `Vector`, or return the provided `default` if the
/// `Vector` is empty.
///
/// Returns a cloned reference to the last element, or the given default value.
pub fn get_last_or(default: PtrValue) -> impl Fn(&Vector) -> PtrValue {
    move |vector| vector.get_last_or(default.clone())
}

/// View the last element of a `Vector`, or compute a default using the provided
/// function if the `Vector` is empty.
///
/// Returns a cloned reference to the last element, or the result of calling the
/// provided function.
pub fn get_last_or_else(else_fn: impl Fn(&Vector) -> PtrValue) -> impl Fn(&Vector) -> PtrValue {
    move |vector| vector.get_last_or_else(|vector| else_fn(vector))
}

/// View the element at index `n` of a `Vector`, or `None` if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or `None` if
/// the index is out of bounds.
pub fn get_nth(n: usize) -> impl Fn(&Vector) -> Option<PtrValue> {
    move |vector| vector.get_nth(n)
}

/// View the element at index `n` of a `Vector`, or `None` if out of bounds.
///
/// Returns a reference to the element at the given index, or `None` if the
/// index is out of bounds.
pub fn get_nth_ref(n: usize) -> impl Fn(&Vector) -> Option<&Value> {
    move |vector| vector.get_nth_ref(n)
}

/// View the element at index `n` of a `Vector`, or return the provided `default`
/// if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or the given
/// default value if out of bounds.
pub fn get_nth_or(n: usize, default: PtrValue) -> impl Fn(&Vector) -> PtrValue {
    move |vector| vector.get_nth_or(n, default.clone())
}

/// View the element at index `n` of a `Vector`, or `nil` if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or `nil` as a
/// default if out of bounds.
pub fn get_nth_or_nil(n: usize) -> impl Fn(&Vector) -> PtrValue {
    move |vector| vector.get_nth_or_nil(n)
}

/// View the element at index `n` of a `Vector`, or compute a default using the
/// provided function if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or the result
/// of calling the provided function if out of bounds.
pub fn get_nth_or_else(
    n: usize,
    else_fn: impl Fn(&Vector) -> PtrValue,
) -> impl Fn(&Vector) -> PtrValue {
    move |vector| vector.get_nth_or_else(n, |vector| else_fn(vector))
}
