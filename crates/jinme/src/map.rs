use crate::prelude::*;
use ::core::fmt;
use ::std::sync::Arc;
use itertools::Itertools as _;

/// Alias for `Arc<Map>` - a reference-counted pointer to a Map.
pub type PtrMap = Arc<Map>;

/// Internal type alias for the map implementation using im::HashMap.
pub(crate) type MapInner = im::HashMap<PtrValue, PtrValue>;

/// Represents a persistent map using `im::HashMap` for efficient immutable storage.
///
/// Maps are used for key-value pairs, such as for metadata, configuration, and
/// data structures.
///
/// # Immutability
///
/// Maps use `im::HashMap` which provides structural sharing, allowing efficient
/// operations like `assoc` and `dissoc` without cloning the entire map.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let map = Map::new(vec![
///     (Value::symbol_unqualified("a").into(), Value::integer_ptr(1)),
///     (Value::symbol_unqualified("b").into(), Value::integer_ptr(2)),
/// ]);
/// assert_eq!(map.get(&Value::symbol_unqualified("a").into()), Some(Value::integer_ptr(1)));
/// ```
#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Map(MapInner);

impl Map {
    pub fn new_empty() -> Self {
        Self(MapInner::new())
    }

    pub fn new_empty_value() -> Value {
        Value::map(Self(MapInner::new()))
    }

    pub fn new_empty_value_ptr() -> PtrValue {
        Value::map_ptr(Self(MapInner::new()))
    }

    pub fn new(entries: Vec<(PtrValue, PtrValue)>) -> Self {
        let mut map = MapInner::new();
        for (key, value) in entries {
            map.insert(key, value);
        }
        Self(map)
    }

    pub fn new_value(entries: Vec<(PtrValue, PtrValue)>) -> Value {
        Value::map(Self::new(entries))
    }

    pub fn new_value_ptr(entries: Vec<(PtrValue, PtrValue)>) -> PtrValue {
        Value::map_ptr(Self::new(entries))
    }

    pub fn insert(&mut self, key: PtrValue, value: PtrValue) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    pub fn assoc(&self, key: PtrValue, value: PtrValue) -> Self {
        let mut new_map = self.0.clone();
        new_map.insert(key, value);
        Self(new_map)
    }

    pub fn get(&self, key: &PtrValue) -> Option<PtrValue> {
        self.0.get(key).cloned()
    }

    pub fn get_or(&self, key: &PtrValue, or: PtrValue) -> PtrValue {
        self.0.get(key).cloned().unwrap_or(or)
    }

    pub fn get_or_nil(&self, key: &PtrValue) -> PtrValue {
        self.0.get(key).cloned().unwrap_or_else(Value::nil_ptr)
    }

    pub fn get_or_panic(&self, key: &PtrValue) -> PtrValue {
        self.0
            .get(key)
            .cloned()
            .unwrap_or_else(|| panic!("Key not found in Map: {}", key))
    }

    pub fn keys(&self) -> Vec<PtrValue> {
        self.0.iter().map(|(k, _v)| (*k).clone()).collect()
    }

    pub fn values(&self) -> Vec<PtrValue> {
        self.0.iter().map(|(_k, v)| (*v).clone()).collect()
    }

    pub fn contains_key(&self, key: &PtrValue) -> bool {
        self.0.contains_key(key)
    }

    pub fn remove(&mut self, key: &PtrValue) -> &mut Self {
        self.0.remove(key);
        self
    }

    pub fn dissoc(&self, key: &PtrValue) -> Self {
        let mut new_map = self.0.clone();
        new_map.remove(key);
        Self(new_map)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_value(self) -> Value {
        Value::map(self)
    }

    pub fn into_value_ptr(self) -> PtrValue {
        Value::map_ptr(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PtrValue, &PtrValue)> {
        self.0.iter()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|(k, v)| format!("{} {}", k, v))
                .join(", ")
        )
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Map([{}])",
            self.0
                .iter()
                .map(|(k, v)| format!("[{:?}, {:?}]", k, v))
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_empty_map() {
        let map = Map::new_empty();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn new_with_entries() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
        ]);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn new_value() {
        let val = Map::new_value(vec![(
            Value::keyword_unqualified_ptr("x"),
            Value::integer_ptr(10),
        )]);
        assert!(val.is_map());
        if let Value::Map(m, _) = val {
            assert_eq!(m.len(), 1);
        } else {
            panic!("Expected Map variant");
        }
    }

    // ============================================================================
    // INSERT TESTS: Demonstrate mutate-in-place semantics
    //
    // The `insert` method mutates the map in-place via `MapInner`'s transparent
    // copy-on-write mechanism and returns `&mut Self` for chaining. The original
    // map binding reflects all mutations. This is useful for building maps
    // imperatively but differs from `assoc` which returns a new map without
    // side effects.
    // ============================================================================


    #[test]
    fn insert_adds_new_key_value_pair() {
        let mut map = Map::new_empty();
        let key = Value::keyword_unqualified_ptr("name");
        let val = Value::string_ptr("Alice".to_string());
        map.insert(key, val);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_replaces_value_for_existing_key_by_value_equality() {
        let mut map = Map::new_empty();
        let key1 = Value::keyword_unqualified_ptr("key");
        let key2 = Value::keyword_unqualified_ptr("key");
        map.insert(key1, Value::integer_ptr(100));
        map.insert(key2, Value::integer_ptr(200));
        // Should have 1 entry with value 200
        assert_eq!(map.len(), 1);
        assert_eq!(
            *map.get(&Value::keyword_unqualified_ptr("key")).unwrap(),
            Value::integer(200)
        );
    }

    #[test]
    fn insert_replaces_value_for_existing_key_by_pointer_equality() {
        let mut map = Map::new_empty();
        let key = Value::keyword_unqualified_ptr("key");
        map.insert(key.clone(), Value::integer_ptr(100));
        map.insert(key.clone(), Value::integer_ptr(200));
        // Should have 1 entry with value 200
        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(&key).unwrap(), Value::integer(200));
    }

    #[test]
    fn insert_multiple_distinct_keys() {
        let mut map = Map::new_empty();
        map.insert(Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1));
        map.insert(Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2));
        map.insert(Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn get_returns_some_for_present_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("answer"),
            Value::integer_ptr(42),
        )]);
        let result = map.get(&Value::keyword_unqualified_ptr("answer"));
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), Value::integer(42));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let result = map.get(&Value::keyword_unqualified_ptr("b"));
        assert!(result.is_none());
    }

    #[test]
    fn get_or_returns_default_for_missing_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let default = Value::integer_ptr(999);
        let result = map.get_or(&Value::keyword_unqualified_ptr("b"), default.clone());
        assert_eq!(*result, *default);
    }

    #[test]
    fn get_or_nil_returns_nil_for_missing_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let result = map.get_or_nil(&Value::keyword_unqualified_ptr("b"));
        assert!(result.is_nil());
    }

    #[test]
    fn get_or_panic_panics_on_missing_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = map.get_or_panic(&Value::keyword_unqualified_ptr("b"));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn keys_returns_all_keys() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
            (Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3)),
        ]);
        let keys = map.keys();
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn values_returns_all_values() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
            (Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3)),
        ]);
        let values = map.values();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn contains_key_returns_true_for_present_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("present"),
            Value::integer_ptr(1),
        )]);
        assert!(map.contains_key(&Value::keyword_unqualified_ptr("present")));
    }

    #[test]
    fn contains_key_returns_false_for_missing_key() {
        let map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        assert!(!map.contains_key(&Value::keyword_unqualified_ptr("b")));
    }

    #[test]
    fn remove_removes_entry_and_mutates_map() {
        let mut map = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
        ]);
        let initial_len = map.len();
        map.remove(&Value::keyword_unqualified_ptr("b"));
        assert_eq!(map.len(), initial_len - 1);
        assert!(!map.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(map.contains_key(&Value::keyword_unqualified_ptr("a")));
    }

    #[test]
    fn remove_on_non_existent_key_does_nothing() {
        let mut map = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let initial_len = map.len();
        map.remove(&Value::keyword_unqualified_ptr("b"));
        assert_eq!(map.len(), initial_len);
    }

    #[test]
    fn equality_with_same_entries() {
        let map1 = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
        ]);
        let map2 = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
        ]);
        assert_eq!(map1, map2);
    }

    // ============================================================================
    // ASSOC TESTS: Demonstrate immutability semantics
    //
    // Unlike `insert` which mutates the original map in-place (returns `&mut Self`),
    // `assoc` does NOT mutate the original map but returns a NEW map with the updated
    // key-value pair. This immutability pattern mirrors Clojure's `assoc` semantics
    // and is the foundation for implementing `clojure.core/assoc`.
    // ============================================================================


    #[test]
    fn assoc_creates_new_map_without_mutating_original() {
        let original = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
        ]);
        let original_len = original.len();

        let updated = original.assoc(Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3));

        // Original map is unchanged
        assert_eq!(original.len(), original_len);
        assert!(!original.contains_key(&Value::keyword_unqualified_ptr("c")));

        // New map has the additional key
        assert_eq!(updated.len(), 3);
        assert!(updated.contains_key(&Value::keyword_unqualified_ptr("c")));
        assert_eq!(
            *updated.get(&Value::keyword_unqualified_ptr("c")).unwrap(),
            Value::integer(3)
        );

        // Maps are NOT equivalent (have different contents)
        assert_ne!(&original, &updated);
    }

    #[test]
    fn assoc_replaces_value_for_existing_key_without_mutation() {
        let original = Map::new(vec![(
            Value::keyword_unqualified_ptr("key"),
            Value::integer_ptr(100),
        )]);
        let original_value = original
            .get(&Value::keyword_unqualified_ptr("key"))
            .unwrap();

        let updated = original.assoc(
            Value::keyword_unqualified_ptr("key"),
            Value::integer_ptr(200),
        );

        // Original map unchanged: still has the old value
        assert_eq!(
            *original
                .get(&Value::keyword_unqualified_ptr("key"))
                .unwrap(),
            Value::integer(100)
        );
        assert_eq!(
            *original
                .get(&Value::keyword_unqualified_ptr("key"))
                .unwrap(),
            *original_value
        );

        // New map has the updated value
        assert_eq!(
            *updated.get(&Value::keyword_unqualified_ptr("key")).unwrap(),
            Value::integer(200)
        );

        // Size is same (1 key)
        assert_eq!(original.len(), 1);
        assert_eq!(updated.len(), 1);
    }

    #[test]
    fn assoc_multiple_updates_creates_independent_maps() {
        let original = Map::new_empty();

        // Chain multiple assoc calls
        let map1 = original.assoc(Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1));
        let map2 = map1.assoc(Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2));
        let map3 = map2.assoc(Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3));

        // Original is untouched
        assert_eq!(original.len(), 0);

        // Each intermediate result is independent
        assert_eq!(map1.len(), 1);
        assert!(map1.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(!map1.contains_key(&Value::keyword_unqualified_ptr("b")));

        assert_eq!(map2.len(), 2);
        assert!(map2.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(map2.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(!map2.contains_key(&Value::keyword_unqualified_ptr("c")));

        assert_eq!(map3.len(), 3);
        assert!(map3.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(map3.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(map3.contains_key(&Value::keyword_unqualified_ptr("c")));

        // All maps are NOT equivalent (have different contents)
        assert_ne!(&original, &map1);
        assert_ne!(&map1, &map2);
        assert_ne!(&map2, &map3);
    }

    #[test]
    fn assoc_with_multiple_distinct_keys() {
        let original = Map::new_empty();
        assert_eq!(original.len(), 0);

        let result = original
            .assoc(Value::keyword_unqualified_ptr("x"), Value::integer_ptr(10))
            .assoc(Value::keyword_unqualified_ptr("y"), Value::integer_ptr(20))
            .assoc(Value::keyword_unqualified_ptr("z"), Value::integer_ptr(30));

        // Original still empty
        assert_eq!(original.len(), 0);

        // Result has all three keys
        assert_eq!(result.len(), 3);
        assert_eq!(
            *result.get(&Value::keyword_unqualified_ptr("x")).unwrap(),
            Value::integer(10)
        );
        assert_eq!(
            *result.get(&Value::keyword_unqualified_ptr("y")).unwrap(),
            Value::integer(20)
        );
        assert_eq!(
            *result.get(&Value::keyword_unqualified_ptr("z")).unwrap(),
            Value::integer(30)
        );
    }

    // ============================================================================
    // DISSOC TESTS: Demonstrate immutability semantics (inverse of assoc)
    //
    // The `dissoc` method removes a key from the map without mutating the
    // original, returning a NEW map with the key removed. This mirrors `assoc`
    // semantics but for deletion rather than insertion. Meanwhile, `remove` is
    // the mutable counterpart that modifies the map in-place and returns `&mut Self`
    // for chaining.
    // ============================================================================


    #[test]
    fn dissoc_removes_key_without_mutating_original() {
        let original = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
            (Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3)),
        ]);
        let original_len = original.len();

        let updated = original.dissoc(&Value::keyword_unqualified_ptr("b"));

        // Original map is unchanged
        assert_eq!(original.len(), original_len);
        assert!(original.contains_key(&Value::keyword_unqualified_ptr("b")));

        // New map has the key removed
        assert_eq!(updated.len(), original_len - 1);
        assert!(!updated.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(updated.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(updated.contains_key(&Value::keyword_unqualified_ptr("c")));

        // Maps are NOT equivalent (have different contents)
        assert_ne!(&original, &updated);
    }

    #[test]
    fn dissoc_on_non_existent_key_without_mutation() {
        let original = Map::new(vec![(
            Value::keyword_unqualified_ptr("a"),
            Value::integer_ptr(1),
        )]);
        let original_len = original.len();

        let updated = original.dissoc(&Value::keyword_unqualified_ptr("missing"));

        // Original map is unchanged
        assert_eq!(original.len(), original_len);

        // New map is also unchanged (key didn't exist)
        assert_eq!(updated.len(), original_len);
        assert!(updated.contains_key(&Value::keyword_unqualified_ptr("a")));
    }

    #[test]
    fn dissoc_multiple_removals_creates_independent_maps() {
        let original = Map::new(vec![
            (Value::keyword_unqualified_ptr("a"), Value::integer_ptr(1)),
            (Value::keyword_unqualified_ptr("b"), Value::integer_ptr(2)),
            (Value::keyword_unqualified_ptr("c"), Value::integer_ptr(3)),
        ]);

        // Chain multiple dissoc calls
        let map1 = original.dissoc(&Value::keyword_unqualified_ptr("c"));
        let map2 = map1.dissoc(&Value::keyword_unqualified_ptr("b"));
        let map3 = map2.dissoc(&Value::keyword_unqualified_ptr("a"));

        // Original is untouched
        assert_eq!(original.len(), 3);

        // Each intermediate result is independent
        assert_eq!(map1.len(), 2);
        assert!(!map1.contains_key(&Value::keyword_unqualified_ptr("c")));
        assert!(map1.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(map1.contains_key(&Value::keyword_unqualified_ptr("b")));

        assert_eq!(map2.len(), 1);
        assert!(!map2.contains_key(&Value::keyword_unqualified_ptr("c")));
        assert!(!map2.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(map2.contains_key(&Value::keyword_unqualified_ptr("a")));

        assert_eq!(map3.len(), 0);
        assert!(!map3.contains_key(&Value::keyword_unqualified_ptr("a")));
        assert!(!map3.contains_key(&Value::keyword_unqualified_ptr("b")));
        assert!(!map3.contains_key(&Value::keyword_unqualified_ptr("c")));

        // All maps are NOT equivalent (have different contents)
        assert_ne!(&original, &map1);
        assert_ne!(&map1, &map2);
        assert_ne!(&map2, &map3);
    }

    #[test]
    fn dissoc_with_multiple_distinct_keys() {
        let original = Map::new(vec![
            (Value::keyword_unqualified_ptr("x"), Value::integer_ptr(10)),
            (Value::keyword_unqualified_ptr("y"), Value::integer_ptr(20)),
            (Value::keyword_unqualified_ptr("z"), Value::integer_ptr(30)),
        ]);
        assert_eq!(original.len(), 3);

        let result = original
            .dissoc(&Value::keyword_unqualified_ptr("z"))
            .dissoc(&Value::keyword_unqualified_ptr("x"));

        // Original still has all three keys
        assert_eq!(original.len(), 3);

        // Result has two keys removed
        assert_eq!(result.len(), 1);
        assert!(!result.contains_key(&Value::keyword_unqualified_ptr("x")));
        assert!(!result.contains_key(&Value::keyword_unqualified_ptr("z")));
        assert!(result.contains_key(&Value::keyword_unqualified_ptr("y")));
    }
}
