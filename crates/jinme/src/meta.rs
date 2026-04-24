use crate::prelude::*;
use ::std::sync::Arc;

pub fn new_unset() -> Option<Map> {
    None
}

pub fn new(map: Map) -> Option<Map> {
    Some(map)
}

/// Trait for metadata operations on metadata types.
///
/// This trait provides methods for associating key-value pairs and retrieving values
/// from metadata maps. It's implemented for `Option<Arc<Map>>` to handle optional metadata.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let meta = None::<Arc<Map>>;
/// let meta = meta.assoc(Value::keyword_unqualified("line").into(), Value::integer_ptr(42));
/// ```
pub trait MetaOps {
    /// Insert or update a key-value pair in the metadata map.
    ///
    /// Returns a new `Option<Arc<Map>>` with the updated map (doesn't mutate in place).
    ///
    /// # Arguments
    ///
    /// * `key` - The key to associate
    /// * `value` - The value to associate
    ///
    /// # Returns
    ///
    /// A new `Option<Arc<Map>>` with the updated metadata
    fn assoc(&self, key: PtrValue, value: PtrValue) -> Option<Arc<Map>>;

    /// Retrieve a value by key from the metadata map.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// `Some(value)` if the key exists, `None` otherwise
    fn get(&self, key: &PtrValue) -> Option<PtrValue>;
}

/// Helper function to format metadata for display.
///
/// Returns `"{}"` for `None` or the string representation of the map for `Some`.
pub fn display_meta(meta: &Arc<Option<Map>>) -> String {
    match meta.as_ref() {
        None => "{}".to_string(),
        Some(map) => format!("{}", map),
    }
}

impl MetaOps for Option<Arc<Map>> {
    fn assoc(&self, key: PtrValue, value: PtrValue) -> Option<Arc<Map>> {
        match self {
            None => {
                // No existing map, create a new one
                let mut new_map = Map::new_empty();
                new_map.insert(key, value);
                Some(Arc::new(new_map))
            }
            Some(existing_map) => {
                // Clone existing entries
                let mut entries: Vec<(PtrValue, PtrValue)> = existing_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                // Find and update, or insert if not found
                let mut found = false;
                for (k, v) in entries.iter_mut() {
                    if PtrValue::ptr_eq(k, &key) || *k == key {
                        *v = value.clone();
                        found = true;
                        break;
                    }
                }
                if !found {
                    entries.push((key, value));
                }
                Some(Arc::new(Map::new(entries)))
            }
        }
    }

    fn get(&self, key: &PtrValue) -> Option<PtrValue> {
        self.as_ref().and_then(|map| map.get(key))
    }
}
