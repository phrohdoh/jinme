use crate::prelude::*;
use std::sync::Arc;

/// Extract metadata from a Var, returning a clone of the metadata.
///
/// This returns the Var's own metadata, independent from any Value wrapper's metadata.
pub fn meta(var: &Var) -> Option<Arc<Map>> {
    var.meta()
}

/// View the metadata map as an Option, cloning if present.
///
/// Returns Some(Map) if metadata exists, None otherwise.
pub fn view_meta(var: &Var) -> Option<Map> {
    var.meta().as_ref().map(|rc_map| (**rc_map).clone())
}

/// Replace the Var's entire metadata in-place.
///
/// This mutates the Var's metadata directly via the RefCell.
pub fn set_meta(var: &Var, meta: Option<Arc<Map>>) {
    var.set_meta(meta)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn optics_meta_empty() {
        let var = Var::new_unbound();
        let m = meta(&var);
        assert!(m.as_ref().is_none());
    }

    #[test]
    fn optics_view_meta() {
        let var = Var::new_unbound();
        let key = Arc::new(Value::keyword(Keyword::new_unqualified("key")));
        let value = Arc::new(Value::string("value".to_string()));

        var.assoc_meta(key.clone(), value.clone());

        let viewed = view_meta(&var);
        assert!(viewed.is_some());
        let map = viewed.unwrap();
        assert_eq!(map.get(&key), Some(value));
    }

    #[test]
    fn optics_set_meta() {
        let var = Var::new_unbound();

        // Create new metadata
        let key = Arc::new(Value::keyword(Keyword::new_unqualified("new")));
        let value = Arc::new(Value::string("metadata".to_string()));
        let mut new_map = Map::new_empty();
        new_map.insert(key.clone(), value.clone());
        let new_meta = Some(Arc::new(new_map));

        // Use optics to set
        set_meta(&var, new_meta);

        // Verify
        assert_eq!(var.get_meta(&key), Some(value));
    }

    #[test]
    fn optics_metadata_independence_from_value_wrapper() {
        // When a Var is wrapped in Value::Var, the wrapper captures the Var's current metadata
        // as a snapshot. This is independent from the Var's own metadata.
        let var = Arc::new(Var::new_unbound());

        let key1 = Arc::new(Value::keyword(Keyword::new_unqualified("key1")));
        let value1 = Arc::new(Value::string("value1".to_string()));

        // Set metadata on the Var itself
        var.assoc_meta(key1.clone(), value1.clone());

        // Create a Value::Var wrapper (which captures Var's current metadata)
        let value_var = Value::Var(var.clone(), var.meta());

        // Extract metadata from the wrapper using value optics
        let wrapper_meta = value::optics::preview_meta_ref(&value_var);
        let wrapper_meta = Option::unwrap(wrapper_meta);

        // At creation time, both point to the same metadata
        assert_eq!(wrapper_meta.get(&key1), Some(value1.clone()));

        // Now update the Var's metadata with a new key-value pair
        let key2 = Arc::new(Value::keyword(Keyword::new_unqualified("key2")));
        let value2 = Arc::new(Value::string("value2".to_string()));
        var.assoc_meta(key2.clone(), value2.clone());

        // The Var has both keys
        assert_eq!(var.get_meta(&key1), Some(value1.clone()));
        assert_eq!(var.get_meta(&key2), Some(value2.clone()));

        // The wrapper's metadata still only has key1 (it's a snapshot)
        // Since Value::Var metadata is independent from Var's metadata
        assert_eq!(wrapper_meta.get(&key1), Some(value1));
        assert_eq!(wrapper_meta.get(&key2), None);

        // They are independent - no shared updating
    }

    #[test]
    fn optics_var_in_var_metadata_independence() {
        // Two separate Vars with independent metadata, one bound to another
        let var1 = Arc::new(Var::new_bound(Arc::new(Value::from(42.0))));
        let var2 = Arc::new(Var::new_bound(Arc::new(Value::Var(
            var1.clone(),
            var1.meta(),
        ))));

        let id_key = Arc::new(Value::keyword(Keyword::new_unqualified("id")));
        let var1_id = Arc::new(Value::string("var1".to_string()));
        let var2_id = Arc::new(Value::string("var2".to_string()));

        // Set metadata on each using optics
        var1.assoc_meta(id_key.clone(), var1_id.clone());
        var2.assoc_meta(id_key.clone(), var2_id.clone());

        // Verify independence
        assert_eq!(var1.get_meta(&id_key), Some(var1_id.clone()));
        assert_eq!(var2.get_meta(&id_key), Some(var2_id));

        // var2 is bound to var1, but their metadata remains independent
        if let Some(var2_value_ptr) = var2.deref() {
            if let Value::Var(deref_var, _) = var2_value_ptr.as_ref() {
                assert_eq!(deref_var.get_meta(&id_key), Some(var1_id));
            } else {
                panic!("Expected var2 to be bound to a Var");
            }
        }
    }

    #[test]
    fn optics_view_empty_meta() {
        let var = Var::new_unbound();
        let viewed = view_meta(&var);
        assert!(viewed.is_none());
    }

    #[test]
    fn optics_get_nonexistent_key() {
        let var = Var::new_unbound();
        let key = Arc::new(Value::keyword(Keyword::new_unqualified("missing")));

        let result = var.get_meta(&key);
        assert_eq!(result, None);
    }

    #[test]
    fn optics_assoc_then_view() {
        let var = Var::new_unbound();
        let key = Arc::new(Value::keyword(Keyword::new_unqualified("key")));
        let value = Arc::new(Value::from(100.0));

        var.assoc_meta(key.clone(), value.clone());
        let viewed = view_meta(&var);

        assert!(viewed.is_some());
        let map = viewed.unwrap();
        assert_eq!(map.get(&key), Some(value));
    }

    #[test]
    fn optics_set_then_get() {
        let var = Var::new_unbound();

        // Build metadata and set via optics
        let key1 = Arc::new(Value::keyword(Keyword::new_unqualified("key1")));
        let key2 = Arc::new(Value::keyword(Keyword::new_unqualified("key2")));
        let value1 = Arc::new(Value::string("one".to_string()));
        let value2 = Arc::new(Value::string("two".to_string()));

        let mut map = Map::new_empty();
        map.insert(key1.clone(), value1.clone());
        map.insert(key2.clone(), value2.clone());

        set_meta(&var, Some(Arc::new(map)));

        // Get via optics
        assert_eq!(var.get_meta(&key1), Some(value1));
        assert_eq!(var.get_meta(&key2), Some(value2));
    }
}
