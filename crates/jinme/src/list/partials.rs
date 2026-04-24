use crate::list::List;
use crate::value::{PtrValue, Value};

/// View the last element of a `List`, or return the provided `default` if the
/// `List` is empty.
///
/// Returns a cloned reference to the last element, or the given default value.
pub fn get_last_or(default: PtrValue) -> impl Fn(&List) -> PtrValue {
    move |list| list.get_last_or(default.clone())
}

/// View the last element of a `List`, or compute a default using the provided
/// function if the `List` is empty.
///
/// Returns a cloned reference to the last element, or the result of calling the
/// provided function.
pub fn get_last_or_else(else_fn: impl Fn(&List) -> PtrValue) -> impl Fn(&List) -> PtrValue {
    move |list| list.get_last_or_else(|list| else_fn(list))
}

/// View the element at index `n` of a `List`, or `None` if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or `None` if
/// the index is out of bounds.
pub fn get_nth(n: usize) -> impl Fn(&List) -> Option<PtrValue> {
    move |list| list.get_nth(n)
}

/// View the element at index `n` of a `List`, or `None` if out of bounds.
///
/// Returns a reference to the element at the given index, or `None` if the
/// index is out of bounds.
pub fn get_nth_ref(n: usize) -> impl Fn(&List) -> Option<&Value> {
    move |list| list.get_nth_ref(n)
}

/// View the element at index `n` of a `List`, or return the provided `default`
/// if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or the given
/// default value if out of bounds.
pub fn get_nth_or(n: usize, default: PtrValue) -> impl Fn(&List) -> PtrValue {
    move |list| list.get_nth_or(n, default.clone())
}

/// View the element at index `n` of a `List`, or `nil` if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or `nil` as a
/// default if out of bounds.
pub fn get_nth_or_nil(n: usize) -> impl Fn(&List) -> PtrValue {
    move |list| list.get_nth_or_nil(n)
}

/// View the element at index `n` of a `List`, or compute a default using the
/// provided function if out of bounds.
///
/// Returns a cloned reference to the element at the given index, or the result
/// of calling the provided function if out of bounds.
pub fn get_nth_or_else(
    n: usize,
    else_fn: impl Fn(&List) -> PtrValue,
) -> impl Fn(&List) -> PtrValue {
    move |list| list.get_nth_or_else(n, |list| else_fn(list))
}
