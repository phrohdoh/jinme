use crate::prelude::*;
use ::std::sync::Arc;

/// A prism for unit-like values (`Value::Nil`). This prism can preview whether a `Value` is nil
/// and reconstruct a nil `Value`.
///
/// The `preview` function extracts unit `()` from a `Value::Nil` or returns `None` otherwise.
/// The `preview_ref` function always returns `Some(())` for nil values (there's nothing to reference).
/// The `review` function constructs a new nil `Value`.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let prism = PrismNil::new();
/// assert_eq!(prism.preview(&Value::nil()), Some(()));
/// assert_eq!(prism.review(), Value::nil_ptr());
/// ```
#[derive(Clone)]
pub struct PrismNil<S> {
    /// Function to extract `()` from the source if it matches the expected variant.
    preview: fn(&S) -> Option<()>,
    /// Function to get a reference to the unit value (always succeeds for unit types).
    preview_ref: fn(&S) -> Option<()>,
    /// Function to construct a source from unit.
    review: fn() -> Arc<S>,
}

impl PrismNil<Value> {
    /// Creates a new `PrismNil` with preview, preview_ref, and review functions.
    pub fn new() -> Self {
        Self {
            preview: |v| {
                if let Value::Nil(_) = v {
                    Some(())
                } else {
                    None
                }
            },
            preview_ref: |v| {
                if let Value::Nil(_) = v {
                    Some(())
                } else {
                    None
                }
            },
            review: Value::nil_ptr,
        }
    }

    /// Previews whether a `Value` is nil and extracts unit `()`.
    /// Returns `Some(())` if the value is nil, `None` otherwise.
    pub fn preview(&self, source: &Value) -> Option<()> {
        (self.preview)(source)
    }

    /// Previews whether a `Value` is nil by reference.
    /// For nil values, always returns `Some(())` if the value matches.
    /// This is equivalent to `preview` for unit types.
    pub fn preview_ref<'a>(&self, source: &'a Value) -> Option<()> {
        (self.preview_ref)(source)
    }

    /// Constructs a nil `Value`.
    pub fn review(&self) -> PtrValue {
        (self.review)()
    }
}

/// A prism for accessing a specific variant of a source type and extracting a value.
/// This provides composable lens-like operations for functional value manipulation.
///
/// Type parameters:
/// * `S`: The source type (typically `Value`)
/// * `A`: The target type being extracted/constructed (e.g., `String`, `List`, `Map`)
///
/// The prism stores two extraction functions:
/// * `preview`: Returns `Option<A>` by cloning or copying the value
/// * `preview_ref`: Returns `Option<&A>` without cloning
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let bool_prism = Prism::new(
///     |v| if let Value::Boolean(b, _) = v { Some(*b) } else { None },
///     |v| if let Value::Boolean(b, _) = v { Some(b) } else { None },
///     |b| Arc::new(Value::Boolean(b, None))
/// );
/// ```
#[derive(Clone)]
pub struct Prism<S, A> {
    /// Function that extracts `A` from the source if it matches the expected variant.
    /// Returns `None` if the source doesn't match.
    preview: fn(&S) -> Option<A>,
    /// Function that extracts `&A` from the source if it matches the expected variant.
    /// Returns `None` if the source doesn't match. The returned reference is valid as long
    /// as the source reference is valid.
    preview_ref: fn(&S) -> Option<&A>,
    /// Function that constructs a source from a value of type `A`.
    review: fn(A) -> Arc<S>,
}

impl<A> Prism<Value, A> {
    /// Creates a new `Prism` with preview, preview_ref, and review functions.
    ///
    /// # Arguments
    /// * `preview`: Function that extracts `A` by value if the value matches
    /// * `preview_ref`: Function that extracts `&A` by reference if the value matches
    /// * `review`: Function that constructs a `Value` from `A`
    pub fn new(
        preview: fn(&Value) -> Option<A>,
        preview_ref: fn(&Value) -> Option<&A>,
        review: fn(A) -> PtrValue,
    ) -> Self {
        Self {
            preview,
            preview_ref,
            review,
        }
    }

    /// Previews the source and extracts a value by cloning/copying if it matches.
    /// Returns `Some(a)` if the source matches the expected variant, `None` otherwise.
    pub fn preview(&self, source: &Value) -> Option<A> {
        (self.preview)(source)
    }

    /// Previews the source and extracts a reference to the value without cloning.
    /// Returns `Some(&a)` if the source matches the expected variant, `None` otherwise.
    /// The reference is valid as long as the source is valid.
    pub fn preview_ref<'a>(&self, source: &'a Value) -> Option<&'a A> {
        (self.preview_ref)(source)
    }

    /// Constructs a source from a value.
    pub fn review(&self, a: A) -> PtrValue {
        (self.review)(a)
    }

    /// Applies a function to the extracted value, if the source matches.
    /// Returns a new `Value` with the modified value, or the original if it doesn't match.
    pub fn modify<F>(&self, source: PtrValue, f: F) -> PtrValue
    where
        F: Fn(A) -> A,
    {
        match self.preview(source.as_ref()) {
            Some(a) => self.review(f(a)),
            None => source,
        }
    }

    /// Sets the value in the source, returning a new `Value`.
    /// Returns a new `Value` with the provided value if the source matches,
    /// or the original if it doesn't match.
    pub fn set(&self, source: PtrValue, a: A) -> PtrValue
    where
        A: Clone,
    {
        self.modify(source, |_| a.clone())
    }

    /// Attempts to apply a function to the extracted value.
    /// Returns `Ok(modified_value)` if the source matches, `Err(original_value)` otherwise.
    pub fn try_modify<F>(&self, source: PtrValue, f: F) -> Result<PtrValue, PtrValue>
    where
        F: Fn(A) -> A,
    {
        match self.preview(source.as_ref()) {
            Some(a) => Ok(self.review(f(a))),
            None => Err(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prism_nil_preview() {
        let prism = PrismNil::new();

        let nil_val = Value::Nil(None);
        assert_eq!(prism.preview(&nil_val), Some(()));

        let int_val = Value::Integer(42, None);
        assert_eq!(prism.preview(&int_val), None);
    }

    #[test]
    fn test_prism_nil_preview_ref() {
        let prism = PrismNil::new();

        let nil_val = Value::Nil(None);
        assert_eq!(prism.preview_ref(&nil_val), Some(()));

        let int_val = Value::Integer(42, None);
        assert_eq!(prism.preview_ref(&int_val), None);
    }

    #[test]
    fn test_prism_nil_review() {
        let prism = PrismNil::new();

        let reviewed = prism.review();
        assert!(matches!(*reviewed, Value::Nil(_)));
    }

    #[test]
    fn test_prism_preview_copy_type() {
        let prism: Prism<Value, i64> = Prism::new(
            |v| {
                if let Value::Integer(i, _) = v {
                    Some(*i)
                } else {
                    None
                }
            },
            |v| {
                if let Value::Integer(_, _) = v {
                    None
                } else {
                    None
                }
            },
            |i| Arc::new(Value::Integer(i, None)),
        );

        let int_val = Value::Integer(42, None);
        assert_eq!(prism.preview(&int_val), Some(42));
        assert_eq!(prism.preview_ref(&int_val), None);
    }

    #[test]
    fn test_prism_preview_non_copy_type() {
        let prism: Prism<Value, String> = Prism::new(
            |v| {
                if let Value::String(s, _) = v {
                    Some(s.clone())
                } else {
                    None
                }
            },
            |v| {
                if let Value::String(s, _) = v {
                    Some(s)
                } else {
                    None
                }
            },
            |s| Arc::new(Value::String(s, None)),
        );

        let str_val = Value::String("hello".to_string(), None);
        assert_eq!(prism.preview(&str_val), Some("hello".to_string()));
        assert_eq!(prism.preview_ref(&str_val), Some(&"hello".to_string()));
    }

    #[test]
    fn test_prism_modify() {
        let prism: Prism<Value, i64> = Prism::new(
            |v| {
                if let Value::Integer(i, _) = v {
                    Some(*i)
                } else {
                    None
                }
            },
            |v| {
                if let Value::Integer(_, _) = v {
                    None
                } else {
                    None
                }
            },
            |i| Arc::new(Value::Integer(i, None)),
        );

        let val = Arc::new(Value::Integer(5, None));
        let modified = prism.modify(val, |i| i * 2);
        assert_eq!(prism.preview(&modified), Some(10));
    }

    #[test]
    fn test_prism_modify_type_mismatch() {
        let prism: Prism<Value, i64> = Prism::new(
            |v| {
                if let Value::Integer(i, _) = v {
                    Some(*i)
                } else {
                    None
                }
            },
            |v| {
                if let Value::Integer(_, _) = v {
                    None
                } else {
                    None
                }
            },
            |i| Arc::new(Value::Integer(i, None)),
        );

        let val = Arc::new(Value::String("hello".to_string(), None));
        let modified = prism.modify(val.clone(), |i| i * 2);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_prism_try_modify_success() {
        let prism: Prism<Value, i64> = Prism::new(
            |v| {
                if let Value::Integer(i, _) = v {
                    Some(*i)
                } else {
                    None
                }
            },
            |v| {
                if let Value::Integer(_, _) = v {
                    None
                } else {
                    None
                }
            },
            |i| Arc::new(Value::Integer(i, None)),
        );

        let val = Arc::new(Value::Integer(5, None));
        let result = prism.try_modify(val, |i| i * 2);
        assert!(result.is_ok());
        assert_eq!(prism.preview(&result.unwrap()), Some(10));
    }

    #[test]
    fn test_prism_try_modify_failure() {
        let prism: Prism<Value, i64> = Prism::new(
            |v| {
                if let Value::Integer(i, _) = v {
                    Some(*i)
                } else {
                    None
                }
            },
            |v| {
                if let Value::Integer(_, _) = v {
                    None
                } else {
                    None
                }
            },
            |i| Arc::new(Value::Integer(i, None)),
        );

        let val = Arc::new(Value::String("hello".to_string(), None));
        let result = prism.try_modify(val.clone(), |i| i * 2);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), val);
    }
}
