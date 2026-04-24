use crate::prelude::*;
use ::core::fmt;
use itertools::Itertools;

pub mod optics;
pub mod partials;

/// Represents a linked list using `im::Vector` for persistent immutable storage.
///
/// Lists are used for ordered collections where the head and tail are important,
/// such as in function argument lists and Clojure-style sequences.
///
/// # Immutability
///
/// Lists use `im::Vector` which provides structural sharing, allowing efficient
/// operations like `rest` without cloning the entire list.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let list = List::from(vec![Value::integer_ptr(1), Value::integer_ptr(2), Value::integer_ptr(3)]);
/// assert_eq!(list.len(), 3);
/// assert_eq!(list.get_first(), Some(Value::integer_ptr(1)));
/// ```
#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct List(im::Vector<PtrValue>);

impl From<Vec<PtrValue>> for List {
    fn from(elements: Vec<PtrValue>) -> Self {
        Self(im::Vector::from(elements))
    }
}

impl List {
    pub fn new_empty() -> Self {
        Self(im::Vector::new())
    }

    pub fn new_empty_value() -> Value {
        Value::list(Self(im::Vector::new()))
    }

    pub fn new_empty_value_ptr() -> PtrValue {
        Value::list_ptr(Self(im::Vector::new()))
    }

    pub fn new_value(elements: Vec<PtrValue>) -> Value {
        Value::list(Self::from(elements))
    }

    pub fn new_value_ptr(elements: Vec<PtrValue>) -> PtrValue {
        Value::list_ptr(Self::from(elements))
    }

    pub fn into_value(self) -> Value {
        Value::list(self)
    }

    pub fn into_value_ptr(self) -> PtrValue {
        Value::list_ptr(self)
    }

    pub fn push_front(&mut self, value: PtrValue) {
        self.0.push_front(value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn rest(&self) -> Self {
        Self(self.0.iter().skip(1).cloned().collect())
    }

    pub fn collect_rest<C>(&self) -> C
    where
        C: FromIterator<PtrValue>,
    {
        self.0.iter().skip(1).cloned().collect()
    }
}

impl List {
    pub fn get_first(&self) -> Option<PtrValue> {
        self.0.front().cloned()
    }

    pub fn get_first_ref(&self) -> Option<&Value> {
        self.0.front().map(PtrValue::as_ref)
    }

    pub fn get_first_or(&self, default: PtrValue) -> PtrValue {
        self.0.front().cloned().unwrap_or(default)
    }

    pub fn get_first_or_nil(&self) -> PtrValue {
        self.0.front().cloned().unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_first_or_else(&self, else_fn: impl FnOnce(&Self) -> PtrValue) -> PtrValue {
        self.0.front().cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_first_or_panic(&self) -> PtrValue {
        self.0.front().cloned().unwrap()
    }

    pub fn get_second(&self) -> Option<PtrValue> {
        self.0.get(1).cloned()
    }

    pub fn get_second_ref(&self) -> Option<&Value> {
        self.0.get(1).map(PtrValue::as_ref)
    }

    pub fn get_second_or(&self, default: PtrValue) -> PtrValue {
        self.0.get(1).cloned().unwrap_or(default)
    }

    pub fn get_second_or_nil(&self) -> PtrValue {
        self.0.get(1).cloned().unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_second_or_else(&self, else_fn: impl FnOnce(&Self) -> PtrValue) -> PtrValue {
        self.0.get(1).cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_second_or_panic(&self) -> PtrValue {
        self.0.get(1).cloned().unwrap()
    }

    pub fn get_last(&self) -> Option<PtrValue> {
        self.0.last().cloned()
    }

    pub fn get_last_ref(&self) -> Option<&Value> {
        self.0.last().map(PtrValue::as_ref)
    }

    pub fn get_last_or(&self, default: PtrValue) -> PtrValue {
        self.0.last().cloned().unwrap_or(default)
    }

    pub fn get_last_or_nil(&self) -> PtrValue {
        self.0.last().cloned().unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_last_or_else(&self, else_fn: impl FnOnce(&Self) -> PtrValue) -> PtrValue {
        self.0.last().cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_last_or_panic(&self) -> PtrValue {
        self.0.last().cloned().unwrap()
    }

    pub fn get_nth(&self, n: usize) -> Option<PtrValue> {
        self.0.get(n).cloned()
    }

    pub fn get_nth_ref(&self, n: usize) -> Option<&Value> {
        self.0.get(n).map(PtrValue::as_ref)
    }

    pub fn get_nth_or(&self, n: usize, or: PtrValue) -> PtrValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or(or)
    }

    pub fn get_nth_or_nil(&self, n: usize) -> PtrValue {
        self.0
            .get(n)
            .map(|v| v.to_owned())
            .unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_nth_or_else(&self, n: usize, else_fn: impl FnOnce(&Self) -> PtrValue) -> PtrValue {
        self.0
            .get(n)
            .map(|v| v.to_owned())
            .unwrap_or_else(|| else_fn(self))
    }

    pub fn get_nth_or_panic(&self, n: usize) -> PtrValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PtrValue> {
        self.0.iter()
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.iter().join(" "))
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "List([{}])",
            self.0.iter().map(|x| format!("{:?}", x)).join(", ")
        )
    }
}

// Generate tests for List
#[cfg(test)]
mod tests {
    use ::std::sync::Arc;

    use super::*;
    #[test]
    fn display() {
        let list = List::from(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        assert_eq!(format!("{}", list), "(1 2 3)");
    }

    #[test]
    fn debug() {
        let list = List::from(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        assert_eq!(
            format!("{:?}", list),
            "List([Value::Integer(1), Value::Integer(2), Value::Integer(3)])"
        );
    }

    #[test]
    fn push_front() {
        // arrange
        let mut list = List::new_empty();
        // act
        list.push_front(Value::integer_ptr(1));
        list.push_front(Value::integer_ptr(2));
        list.push_front(Value::integer_ptr(3));
        // assert
        assert_eq!(list.len(), 3);
        let mut iter = list.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
    }

    #[test]
    fn get_nth_or_panic_given_index_in_bounds() {
        // arrange
        let list = List::from(vec![
            /* 0 */ Value::integer_ptr(3),
            /* 1 */ Value::integer_ptr(7),
            /* 2 */ Value::integer_ptr(9),
        ]);
        // act
        let nth_1 = list.get_nth_or_panic(1);
        // assert
        assert_eq!(*nth_1, Value::integer(7));
    }

    #[test]
    #[should_panic]
    fn get_nth_or_panic_given_index_out_of_bounds_panics() {
        for (index, list) in vec![
            (0, List::new_empty()),
            (10, List::new_empty()),
            (1, List::from(vec![Arc::new(Value::nil())])),
        ] {
            let _ = list.get_nth_or_panic(index);
        }
    }

    #[test]
    fn get_nth_or_nil() {
        // arrange
        let list = List::from(vec![
            /* 0 */ Value::keyword_unqualified_ptr("vanilla"),
            /* 1 */ Value::keyword_unqualified_ptr("chocolate"),
            /* 2 */ Value::keyword_unqualified_ptr("strawberry"),
        ]);
        // act
        let nth_3 = list.get_nth_or_nil(3);
        // assert
        assert!(nth_3.is_nil());
    }

    #[test]
    fn get_nth_or() {
        // arrange
        let list = List::from(vec![
            /* 0 */ Value::keyword_unqualified_ptr("red"),
            /* 1 */ Value::keyword_unqualified_ptr("green"),
            /* 2 */ Value::keyword_unqualified_ptr("blue"),
        ]);
        let or_value = Value::keyword_unqualified_ptr("unknown");
        // act
        let nth_5 = list.get_nth_or(5, or_value.clone());
        // assert
        assert_eq!(*nth_5, *or_value);
    }

    #[test]
    fn new_empty_creates_empty_list() {
        let list = List::new_empty();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn new_with_elements() {
        let list = List::from(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        assert_eq!(list.len(), 3);
        let mut iter = list.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
    }

    #[test]
    fn new_empty_value() {
        let val = List::new_empty_value();
        assert!(val.is_list());
        if let Value::List(list, _) = val {
            assert_eq!(list.len(), 0);
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn new_value() {
        let val = List::new_value(vec![Value::integer_ptr(10), Value::integer_ptr(20)]);
        assert!(val.is_list());
        if let Value::List(list, _) = val {
            assert_eq!(list.len(), 2);
            let mut iter = list.0.iter();
            assert_eq!(**iter.next().unwrap(), Value::integer(10));
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn length_increases_with_push_front() {
        let mut list = List::new_empty();
        assert_eq!(list.len(), 0);
        list.push_front(Value::integer_ptr(1));
        assert_eq!(list.len(), 1);
        list.push_front(Value::integer_ptr(2));
        assert_eq!(list.len(), 2);
        list.push_front(Value::integer_ptr(3));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn lifo_ordering_with_push_front() {
        let mut list = List::new_empty();
        list.push_front(Value::integer_ptr(1));
        list.push_front(Value::integer_ptr(2));
        list.push_front(Value::integer_ptr(3));
        // newest (3) should be at front, then 2, then 1
        let mut iter = list.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
    }

    #[test]
    fn multiple_pushes_preserve_order() {
        let mut list = List::new_empty();
        for i in 1..=5 {
            list.push_front(Value::integer_ptr(i));
        }
        // Order should be: 5, 4, 3, 2, 1
        let mut iter = list.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(5));
        assert_eq!(**iter.next().unwrap(), Value::integer(4));
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
    }

    #[test]
    fn equality_with_same_elements() {
        let list1 = List::from(vec![Value::integer_ptr(1), Value::integer_ptr(2)]);
        let list2 = List::from(vec![Value::integer_ptr(1), Value::integer_ptr(2)]);
        assert_eq!(list1, list2);
    }

    #[test]
    fn inequality_with_different_elements() {
        let list1 = List::from(vec![Value::integer_ptr(1)]);
        let list2 = List::from(vec![Value::integer_ptr(2)]);
        assert_ne!(list1, list2);
    }

    #[test]
    fn cloned_list_equals_original() {
        let list1 = List::from(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        let list2 = list1.clone();
        assert_eq!(list1, list2);
    }

    #[test]
    fn into_value() {
        let list = List::from(vec![Value::integer_ptr(42)]);
        let val = list.into_value();
        assert!(val.is_list());
    }

    #[test]
    fn into_value_ptr() {
        let list = List::from(vec![Value::integer_ptr(99)]);
        let rc_val = list.into_value_ptr();
        assert!(rc_val.is_list());
    }
}
