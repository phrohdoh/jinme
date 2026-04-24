use crate::prelude::*;
use ::std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

/// Alias for `Arc<Environment>` - a reference-counted pointer to an Environment.
///
/// Environments manage the global namespace state, including all namespaces,
/// their variables, aliases, imports, and refers.
pub type PtrEnvironment = Arc<Environment>;

/// Type alias for the map of namespaces by their unqualified names.
pub type Namespaces = HashMap<SymbolUnqualified, PtrNamespace>;

/// Manages the global namespace state for the interpreter.
///
/// An `Environment` holds all namespaces and their bindings, providing
/// access to variables across the entire program. It maintains a single
/// current namespace variable that can be used for quick access.
///
/// # Thread Safety
///
/// The environment uses `Mutex` to protect namespace operations, allowing
/// concurrent access from multiple threads. When you need to modify the
/// environment, you should clone the `PtrEnvironment` to get a new handle.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let mut builder = Environment::builder();
/// builder.insert_namespace(Arc::new(Namespace::new_empty("user")));
/// builder.set_current_namespace_var("user", "*ns*");
/// let env = builder.build_ptr();
/// ```
#[derive(Debug)]
pub struct Environment {
    /// Mutex-protected map of all namespaces
    namespaces: Mutex<Namespaces>,
    /// Symbol reference to the current namespace
    current_namespace_var: SymbolQualified,
}

/// Builder for creating an `Environment` with validation.
///
/// The builder provides a fluent API for constructing environments and
/// validates that required fields are set before building.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let mut builder = Environment::builder();
/// builder.insert_namespace(Arc::new(Namespace::new_empty("my-ns")));
/// builder.set_current_namespace_var("my-ns", "*ns*");
/// let env = builder.build_ptr();
/// ```
pub struct EnvironmentBuilder {
    namespaces: Namespaces,
    current_namespace_var: Option<SymbolQualified>,
}

// constructors
impl Environment {
    pub fn builder() -> EnvironmentBuilder {
        EnvironmentBuilder {
            namespaces: Namespaces::new(),
            current_namespace_var: None,
        }
    }
}

impl EnvironmentBuilder {
    pub fn insert_namespace(&mut self, ns: PtrNamespace) -> &mut Self {
        self.namespaces
            .insert(SymbolUnqualified::new(ns.name_str()), ns);
        self
    }

    pub fn set_current_namespace_var(&mut self, ns_name: &str, name: &str) -> &mut Self {
        self.current_namespace_var = Some(SymbolQualified::new(ns_name, name));
        self
    }

    pub fn build_blockers(&self) -> Vec<String> {
        let mut blockers = vec![];
        if self.current_namespace_var.is_none() {
            blockers.push("current_namespace_var is not set".to_owned());
        }
        blockers
    }

    pub fn can_build(&self) -> bool {
        self.build_blockers().is_empty()
    }

    pub fn build(self) -> Environment {
        let blockers = self.build_blockers();
        if !blockers.is_empty() {
            panic!(
                "{}",
                itertools::Itertools::join(&mut blockers.into_iter(), ", ")
            );
        }
        Environment {
            namespaces: Mutex::new(self.namespaces),
            current_namespace_var: self
                .current_namespace_var
                .expect("current_namespace_var not set during environment build"),
        }
    }

    pub fn build_ptr(self) -> PtrEnvironment {
        Arc::new(self.build())
    }
}

// read errors
#[derive(Debug, Clone)]
pub enum GetCurrentNamespaceError {
    NoSuchNamespace(SymbolUnqualified),
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    IncorrectValueType(SymbolQualified),
    IncorrectHandleType(SymbolQualified),
}

impl From<GetHandleError> for GetCurrentNamespaceError {
    fn from(get_handle_error: GetHandleError) -> Self {
        match get_handle_error {
            GetHandleError::NoSuchVar(fqn) => Self::NoSuchVar(fqn),
            GetHandleError::UnboundVar(fqn) => Self::UnboundVar(fqn),
            GetHandleError::IncorrectValueType(fqn) => Self::IncorrectValueType(fqn),
            GetHandleError::IncorrectHandleType(fqn) => Self::IncorrectHandleType(fqn),
        }
    }
}

// reads
impl Environment {
    pub fn current_namespace_var(&self) -> &SymbolQualified {
        &self.current_namespace_var
    }

    pub fn try_get_current_namespace(&self) -> Result<PtrNamespace, GetCurrentNamespaceError> {
        // "clojure.core"
        let ns_name = self.current_namespace_var.namespace();
        // "*ns*"
        let var_name = self.current_namespace_var.name();
        // clojure.core namespace object
        let ns = self.try_get_namespace(ns_name).ok_or_else(|| {
            GetCurrentNamespaceError::NoSuchNamespace(SymbolUnqualified::new(ns_name))
        })?;
        // namespace object that *ns* is bound to
        let current_namespace = ns.try_get_handle::<PtrNamespace>(var_name)?;
        Ok(current_namespace)
    }

    pub fn get_current_namespace_or_panic(&self) -> PtrNamespace {
        self.try_get_current_namespace().expect(
            "failed to get current namespace, check if *ns* is properly bound in clojure.core",
        )
    }

    // pub fn new_empty() -> Self {
    //     Self {
    //         namespaces: Mutex::new(HashMap::new()),
    //     }
    // }

    // pub fn new_empty_ptr() -> Arc<Self> {
    //     Arc::new(Self::new_empty())
    // }

    // pub fn new_from_namespaced_values<'a, 'b, I>(
    //     namespaced_values_iter: I,
    // ) -> Self
    // where
    //     I: IntoIterator<Item = (&'a str, Vec<(&'b str, Value)>)>
    // {
    //     namespaced_values_iter.into_iter()
    //         .fold(
    //             Self::new_empty(),
    //             |env, (ns_name, named_values)| {
    //                 let ns = Namespace::new_from_named_values(
    //                     ns_name,
    //                     named_values,
    //                 );
    //                 env.namespaces.borrow_mut()
    //                     .insert(SymbolUnqualified::new(ns_name), Arc::new(ns));
    //                 env
    //             },
    //         )
    // }
}

// reads
impl Environment {
    // pub fn namespace_names(&self) -> Vec<String> {
    //     self.namespaces.borrow()
    //         .keys()
    //         .map(|sym_u| sym_u.name().to_owned())
    //         .collect()
    // }

    // pub fn namespaces(&self) -> Vec<PtrNamespace> {
    //     self.namespaces.borrow()
    //         .values()
    //         .map(|ns| ns.to_owned())
    //         .collect()
    // }

    // pub fn namespace_handles(&self) -> Vec<Handle> {
    //     self.namespaces.borrow()
    //         .values()
    //         .map(|ns| Handle::new(ns.to_owned()))
    //         .collect()
    // }

    // pub fn namespaces_entry_values(&self) -> Vec<(PtrValue, PtrValue)> {
    //     self.namespaces.borrow()
    //         .iter()
    //         .map(|(sym_u, ns)| (
    //             Arc::new(Value::symbol(Symbol::Unqualified(sym_u.to_owned()))),
    //             Arc::new(Value::handle(Handle::new(ns.to_owned()))),
    //         ))
    //         .collect()
    // }

    // pub fn namespace_map(&self) -> Map {
    //     let entries = self.namespaces_entry_values();
    //     Map::new(entries)
    // }

    // pub fn namespace_map_value(&self) -> PtrValue {
    //     Arc::new(Value::map(self.namespace_map()))
    // }

    // #[tracing::instrument(ret, level = "info")]
    pub fn all_namespaces(&self) -> Vec<PtrNamespace> {
        self.namespaces
            .lock()
            .expect("mutex poisoned while accessing namespaces in all_namespaces")
            .values()
            .cloned()
            .collect()
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn has_namespace(&self, name: &str) -> bool {
        self.try_get_namespace(name).is_some()
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace(&self, name: &str) -> Option<PtrNamespace> {
        self.namespaces
            .lock()
            .expect("mutex poisoned while accessing namespaces in try_get_namespace")
            .get(&SymbolUnqualified::new(name))
            .cloned()
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace_handle(&self, name: &str) -> Option<Handle> {
        self.try_get_namespace(name).map(|ns| Handle::new(ns))
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace_handle_value(&self, name: &str) -> Option<PtrValue> {
        self.try_get_namespace_handle(name)
            .map(|handle| Arc::new(Value::handle(handle)))
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_or_panic(&self, name: &str) -> PtrNamespace {
        self.try_get_namespace(name)
            .expect(&format!("no such namespace: {}", name))
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_handle_or_panic(&self, name: &str) -> Handle {
        self.try_get_namespace_handle(name)
            .expect(&format!("no such namespace: {}", name))
    }

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_handle_value_or_panic(&self, name: &str) -> PtrValue {
        self.try_get_namespace_handle_value(name)
            .expect(&format!("no such namespace: {}", name))
    }
}

// writes
impl Environment {
    /*
    // #[tracing::instrument(ret, level = "info")]
    pub fn bind_value_ptr<'env>(
        &'env self,
        (ns_name, name): (&str, &str),
        value_ptr: PtrValue,
    ) {
        let ns = match self.try_get_namespace(&ns_name) {
            Some(ns) => ns,
            None => {
                self.create_namespace(ns_name);
                self.get_namespace_or_panic(&ns_name)
            },
        };
        match ns.try_get_var(name) {
            Some(var) => { var.as_ref().bind(value_ptr); },
            None => { ns.insert_var(name, Var::new_bound(value_ptr)); },
        }
    }

    // #[tracing::instrument(ret, level = "info")]
    pub fn bind_value<'env>(
        &'env self,
        fqn: (&str, &str),
        value: Value,
    ) {
        let value_ptr = Arc::new(value);
        self.bind_value_ptr(fqn, value_ptr);
    }

    // #[tracing::instrument(ret, level = "info", skip(arities))]
    pub fn bind_function<'env>(
        &'env self,
        (ns_name, name): (&str, &str),
        arities: Vec<(
            FunctionArity,
            Box<dyn Fn(PtrEnvironment, Vec<PtrValue>) -> PtrValue>,
        )>,
    ) -> PtrValue {
        let value = Value::function({
            let mut builder = Function::builder();
            builder.set_name(name.to_owned());
            for (arity, body) in arities {
                builder.add_body(arity, body);
            }
            Arc::new(builder.build())
        });
        let rc_value = Arc::new(value);
        self.bind_value_ptr(
            (ns_name, name),
            rc_value.clone(),
        );
        rc_value
    }
    */

    // #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn create_namespace(&self, name: &str) -> PtrNamespace {
        match self.try_get_namespace(name) {
            Some(ns) => ns,
            None => {
                {
                    self.namespaces
                        .lock()
                        .expect("mutex poisoned while accessing namespaces in create_namespace")
                        .insert(
                            SymbolUnqualified::new(name),
                            Arc::new(Namespace::new_empty(name)),
                        );
                }
                self.get_namespace_or_panic(name)
            }
        }
    }

    pub fn insert_namespace(&self, ns: PtrNamespace) {
        self.namespaces
            .lock()
            .expect("mutex poisoned while accessing namespaces in insert_namespace")
            .insert(SymbolUnqualified::new(ns.name_str()), ns);
    }

    pub fn remove_namespace(&self, name: &str) {
        self.namespaces
            .lock()
            .expect("mutex poisoned while accessing namespaces in remove_namespace")
            .remove(&SymbolUnqualified::new(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_env() -> PtrEnvironment {
        let mut env_builder = Environment::builder();
        env_builder.set_current_namespace_var("clojure.core", "*ns*");
        env_builder.insert_namespace(Namespace::new_empty_ptr("clojure.core"));
        env_builder.build_ptr()
    }

    #[test]
    fn create_nonexistent_namespace() {
        // arrange
        let env = create_env();
        let ns_name = "my-ns";
        let has_ns_before_act = env.has_namespace(&ns_name);
        // act
        env.create_namespace(ns_name);
        // assert
        let has_ns_after_act = env.has_namespace(&ns_name);
        assert!(!has_ns_before_act);
        assert!(has_ns_after_act);
    }

    #[test]
    fn create_existent_namespace_does_not_overwrite() {
        // arrange
        let env = create_env();
        let ns_name = "my-ns";
        let name = "my-var";
        let value = Value::integer(42);
        let var = Var::new_bound(value);
        env.create_namespace(ns_name).insert_var(name, var);
        // act
        let ns = env.create_namespace(ns_name);
        eprintln!("{:?}", ns);
        // assert
        let var = ns.try_get_var(name);
        assert!(var.is_ok(), "var expected to be present");
        let var = var.unwrap();
        let val = var.deref();
        assert!(val.is_some(), "var expected to be bound");
        let val = val.unwrap();
        assert_eq!(*val, Value::integer(42), "value of var is unchanged");
    }

    #[test]
    fn get_var() {
        // arrange
        let ns_name = "my-ns";
        let name = "my-var";
        let value = Value::keyword_unqualified("my-val");
        let env = create_env();
        env.create_namespace(ns_name)
            .insert_var(name, Var::new_bound(value));
        // act
        let ns = env.get_namespace_or_panic("my-ns");
        let var = ns.try_get_var("my-var");
        // assert
        assert!(var.is_ok());
    }
}
