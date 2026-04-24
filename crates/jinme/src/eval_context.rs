use crate::prelude::*;
use im::HashMap;

/// Evaluation context holding local variable bindings.
///
/// The `EvalContext` is used by special forms like `let*`, `fn*`, and `do` to implement
/// lexical scoping. It maintains a map of local variable bindings that are visible
/// only within the scope where they were defined.
///
/// # Thread Safety
///
/// The context uses `im::HashMap` which provides efficient cloning and immutable
/// semantics, making it safe to share across threads without explicit locking.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let ctx = EvalContext::new_empty();
/// let ctx = ctx.with_local("x".to_string(), Value::integer_ptr(42));
/// assert_eq!(ctx.resolve_local("x"), Some(Value::integer_ptr(42)));
/// ```
#[derive(Clone, Debug)]
pub struct EvalContext {
    /// Local variable bindings using im::HashMap for efficient cloning
    locals: HashMap<String, PtrValue>,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl EvalContext {
    /// Create an empty context with no local bindings.
    pub fn new_empty() -> Self {
        EvalContext {
            locals: HashMap::new(),
        }
    }

    /// Insert a local binding into this context. Modifies the context in place.
    pub fn insert_local(&mut self, name: String, value: PtrValue) {
        self.locals.insert(name, value);
    }

    /// Insert multiple local bindings into this context. Modifies the context in place.
    pub fn insert_locals(&mut self, bindings: Vec<(String, PtrValue)>) {
        for (name, value) in bindings {
            self.locals.insert(name, value);
        }
    }

    /// Extend this context with a new local binding, returning a new context.
    /// Does not modify the original context (functional approach).
    pub fn with_local(&self, name: impl Into<String>, value: impl Into<PtrValue>) -> Self {
        let new_locals = self.locals.update(name.into(), value.into());
        Self { locals: new_locals }
    }

    /// Extend this context with multiple local bindings, returning a new context.
    /// Does not modify the original context (functional approach).
    pub fn with_locals(&self, bindings: Vec<(String, PtrValue)>) -> Self {
        let mut new_locals = self.locals.clone();
        for (name, value) in bindings {
            new_locals.insert(name, value);
        }
        Self { locals: new_locals }
    }

    /// Look up a local binding by name.
    pub fn resolve_local(&self, name: &str) -> Option<PtrValue> {
        self.locals.get(name).cloned()
    }

    /// Check if a local binding exists.
    pub fn has_local(&self, name: &str) -> bool {
        self.locals.contains_key(name)
    }

    /// Get the number of local bindings in this context.
    pub fn len(&self) -> usize {
        self.locals.len()
    }

    /// Check if this context has no local bindings.
    pub fn is_empty(&self) -> bool {
        self.locals.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_context() {
        let ctx = EvalContext::new_empty();
        assert!(ctx.is_empty());
        assert_eq!(ctx.len(), 0);
    }

    #[test]
    fn with_local() {
        let ctx = EvalContext::new_empty();
        let value = Value::integer_ptr(42);
        let ctx2 = ctx.with_local("x".to_string(), value.clone());

        assert!(ctx.is_empty());
        assert!(!ctx2.is_empty());
        assert_eq!(ctx2.len(), 1);
        assert_eq!(ctx2.resolve_local("x"), Some(value));
    }

    #[test]
    fn with_locals_batch() {
        let ctx = EvalContext::new_empty();
        let val1 = Value::integer_ptr(1);
        let val2 = Value::integer_ptr(2);
        let bindings = vec![
            ("a".to_string(), val1.clone()),
            ("b".to_string(), val2.clone()),
        ];

        let ctx2 = ctx.with_locals(bindings);
        assert_eq!(ctx2.len(), 2);
        assert_eq!(ctx2.resolve_local("a"), Some(val1));
        assert_eq!(ctx2.resolve_local("b"), Some(val2));
    }

    #[test]
    fn shadowing() {
        let ctx = EvalContext::new_empty();
        let val1 = Value::integer_ptr(1);
        let val2 = Value::integer_ptr(2);

        let ctx2 = ctx.with_local("x".to_string(), val1.clone());
        let ctx3 = ctx2.with_local("x".to_string(), val2.clone());

        assert_eq!(ctx2.resolve_local("x"), Some(val1));
        assert_eq!(ctx3.resolve_local("x"), Some(val2));
    }
}
