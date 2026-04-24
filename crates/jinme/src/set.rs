use crate::prelude::*;
use ::core::fmt;
use itertools::Itertools as _;

/// Represents a persistent set using `im::OrdSet` for efficient immutable storage.
///
/// Sets are used for collections where uniqueness is important, such as for
/// deduplication and membership testing.
///
/// # Immutability
///
/// Sets use `im::OrdSet` which provides structural sharing, allowing efficient
/// operations like `insert` without cloning the entire set.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let set = Set::new(vec![Value::integer_ptr(1), Value::integer_ptr(2), Value::integer_ptr(2)]);
/// assert_eq!(set.len(), 2);
/// assert!(set.contains(&Value::integer_ptr(1)));
/// ```
#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Set(im::OrdSet<PtrValue>);

impl Set {
    pub fn new_empty() -> Self {
        Self(im::OrdSet::new())
    }

    pub fn new_empty_value() -> Value {
        Value::set(Self(im::OrdSet::new()))
    }

    pub fn new_empty_value_ptr() -> PtrValue {
        Value::set_ptr(Self(im::OrdSet::new()))
    }

    pub fn new(elements: Vec<PtrValue>) -> Self {
        let mut set = im::OrdSet::new();
        for elem in elements {
            set.insert(elem);
        }
        Self(set)
    }

    pub fn new_value(elements: Vec<PtrValue>) -> Value {
        Value::set(Self::new(elements))
    }

    pub fn new_value_ptr(elements: Vec<PtrValue>) -> PtrValue {
        Value::set_ptr(Self::new(elements))
    }

    pub fn insert(&mut self, value: PtrValue) {
        self.0.insert(value);
    }

    pub fn get(&self, value: &PtrValue) -> Option<PtrValue> {
        self.0
            .iter()
            .find(|v| PtrValue::ptr_eq(v, value) || *v == value)
            .cloned()
    }

    pub fn get_or(&self, value: &PtrValue, or: PtrValue) -> PtrValue {
        self.0
            .iter()
            .find(|v| PtrValue::ptr_eq(v, value) || *v == value)
            .cloned()
            .unwrap_or(or)
    }

    pub fn get_or_nil(&self, value: &PtrValue) -> PtrValue {
        self.0
            .iter()
            .find(|v| PtrValue::ptr_eq(v, value) || *v == value)
            .cloned()
            .unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_or_panic(&self, value: &PtrValue) -> PtrValue {
        self.0
            .iter()
            .find(|v| PtrValue::ptr_eq(v, value) || *v == value)
            .cloned()
            .unwrap_or_else(|| panic!("Key not found in Set: {}", value))
    }

    pub fn values(&self) -> Vec<PtrValue> {
        self.0.iter().map(|v| (*v).clone()).collect()
    }

    pub fn contains(&self, value: &PtrValue) -> bool {
        self.0.contains(value)
    }

    pub fn remove(&mut self, value: &PtrValue) -> Option<PtrValue> {
        self.0.remove(value)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &PtrValue> {
        self.0.iter()
    }

    pub fn into_value(self) -> Value {
        Value::set(self)
    }

    pub fn into_value_ptr(self) -> PtrValue {
        Value::set_ptr(self)
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{{{}}}", self.0.iter().join(", "))
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Set([{}])",
            self.0.iter().map(|x| format!("{:?}", x)).join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_empty_set() {
        let set = Set::new_empty();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn new_with_elements() {
        let set = Set::new(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn new_value() {
        let val = Set::new_value(vec![Value::integer_ptr(10), Value::integer_ptr(20)]);
        assert!(val.is_set());
        if let Value::Set(s, _) = val {
            assert_eq!(s.len(), 2);
        } else {
            panic!("Expected Set variant");
        }
    }

    #[test]
    fn insert_adds_new_element() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_ptr(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Value::integer_ptr(1)));
    }

    #[test]
    fn insert_prevents_duplicate_by_value_equality() {
        let mut set = Set::new_empty();
        let val1 = Value::integer_ptr(42);
        let val2 = Value::integer_ptr(42);
        set.insert(val1);
        set.insert(val2);
        // Should still be 1 element
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_prevents_duplicate_by_pointer_equality() {
        let mut set = Set::new_empty();
        let val = Value::integer_ptr(42);
        set.insert(val.clone());
        set.insert(val.clone());
        // Should still be 1 element
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_cannot_insert_same_value_twice() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_ptr(100));
        set.insert(Value::integer_ptr(100));
        set.insert(Value::integer_ptr(100));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_allows_multiple_distinct_values() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_ptr(1));
        set.insert(Value::integer_ptr(2));
        set.insert(Value::integer_ptr(3));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn get_returns_some_for_present_value() {
        let set = Set::new(vec![Value::integer_ptr(42)]);
        let result = set.get(&Value::integer_ptr(42));
        assert!(result.is_some());
    }

    #[test]
    fn get_returns_none_for_missing_value() {
        let set = Set::new(vec![Value::integer_ptr(42)]);
        let result = set.get(&Value::integer_ptr(99));
        assert!(result.is_none());
    }

    #[test]
    fn get_or_nil_returns_nil_for_missing_value() {
        let set = Set::new(vec![Value::integer_ptr(42)]);
        let result = set.get_or_nil(&Value::integer_ptr(99));
        assert!(result.is_nil());
    }

    #[test]
    fn get_or_panic_panics_on_missing_value() {
        let set = Set::new(vec![Value::integer_ptr(42)]);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = set.get_or_panic(&Value::integer_ptr(99));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn remove_removes_element_and_returns_value() {
        let mut set = Set::new(vec![
            Value::integer_ptr(1),
            Value::integer_ptr(2),
            Value::integer_ptr(3),
        ]);
        let removed = set.remove(&Value::integer_ptr(2));
        assert!(removed.is_some());
        assert_eq!(set.len(), 2);
        assert!(!set.contains(&Value::integer_ptr(2)));
    }

    #[test]
    fn remove_returns_none_for_non_existent_value() {
        let mut set = Set::new(vec![Value::integer_ptr(42)]);
        let removed = set.remove(&Value::integer_ptr(99));
        assert!(removed.is_none());
    }

    #[test]
    fn remove_allows_reinsert() {
        let mut set = Set::new(vec![Value::integer_ptr(42)]);
        set.remove(&Value::integer_ptr(42));
        assert_eq!(set.len(), 0);
        set.insert(Value::integer_ptr(42));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Value::integer_ptr(42)));
    }

    #[test]
    fn contains_returns_false_for_non_existent() {
        let set = Set::new(vec![Value::integer_ptr(42)]);
        assert!(!set.contains(&Value::integer_ptr(99)));
    }

    #[test]
    fn equality_with_same_elements() {
        let set1 = Set::new(vec![Value::integer_ptr(1), Value::integer_ptr(2)]);
        let set2 = Set::new(vec![Value::integer_ptr(1), Value::integer_ptr(2)]);
        assert_eq!(set1, set2);
    }
}
