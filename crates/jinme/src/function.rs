use crate::prelude::*;
use ::core::{cmp, fmt, hash, iter};
use ::std::sync::Arc;

/// Wrap a closure into an `PtrDynIFunction` with a specified arity.
///
/// This is a convenience helper for the common case of wrapping a single
/// closure as an `IFunction` implementation. Similar to `box_fn`, but returns
/// `PtrDynIFunction` instead of a boxed function pointer.
///
/// # Example
/// ```
/// # use jinme::prelude::*;
/// let func = closure_fn(FunctionArity::Exactly(2), |env, _ctx, args| {
///     todo!()
/// });
/// ```
pub fn closure_fn(
    arity: FunctionArity,
    func: impl Fn(PtrEnvironment, EvalContext, Vec<PtrValue>) -> PtrValue + Send + Sync + 'static,
) -> (FunctionArity, PtrDynIFunction) {
    (arity, Arc::new(func))
}

/// Build a `Function` from a collection of `IFunction` implementations.
///
/// This is the primary way to construct multi-arity functions when you have
/// custom `IFunction` implementations or need to pass pre-built trait objects.
///
/// # Example with closures
/// ```
/// # use jinme::prelude::*;
/// let func = build_function("my-fn", vec![
///     closure_fn(FunctionArity::Exactly(0), |_env, _ctx, _args| Value::nil_ptr()),
///     closure_fn(FunctionArity::Exactly(1), |_env, _ctx, args| args[0].clone()),
/// ]);
/// ```
///
/// # Example with custom struct
/// ```
/// # use ::std::sync::Arc;
/// # use jinme::prelude::*;
/// struct MyCustomFn { state: i32 }
/// impl IFunction for MyCustomFn {
///     fn invoke(&self, env: PtrEnvironment, _ctx: EvalContext, args: Vec<PtrValue>) -> PtrValue {
///         todo!()
///     }
/// }
///
/// let func = build_function("custom", vec![
///     (FunctionArity::Exactly(1), Arc::new(MyCustomFn { state: 42 })),
/// ]);
/// ```
pub fn build_function(name: &str, arities: Vec<(FunctionArity, PtrDynIFunction)>) -> Function {
    let mut builder = Function::builder();
    builder.set_name(name.to_owned());
    for (arity, body) in arities {
        builder.add_ifunction(arity, body);
    }
    builder.build()
}

pub fn build_function_ptr(
    name: &str,
    arities: Vec<(FunctionArity, PtrDynIFunction)>,
) -> PtrFunction {
    Arc::new(build_function(name, arities))
}

pub fn build_function_value(
    name: &str,
    arities: Vec<(FunctionArity, PtrDynIFunction)>,
) -> Value {
    Value::function(build_function_ptr(name, arities))
}

pub fn build_function_value_ptr(
    name: &str,
    arities: Vec<(FunctionArity, PtrDynIFunction)>,
) -> PtrValue {
    Arc::new(build_function_value(name, arities))
}

/// Trait for function implementations that can be invoked with environment and arguments.
///
/// This trait is implemented for closures and custom function types. It provides
/// a consistent interface for invoking functions with an environment, context,
/// and argument list.
///
/// # Arguments
///
/// * `env` - The environment to use for variable resolution
/// * `ctx` - The evaluation context
/// * `args` - The arguments to pass to the function
///
/// # Returns
///
/// The result of the function invocation as a `PtrValue`
pub trait IFunction: Send + Sync {
    fn invoke(&self, env: PtrEnvironment, ctx: EvalContext, args: Vec<PtrValue>) -> PtrValue;
}

/// Function implementation for closures.
///
/// Automatically implements `IFunction` for any closure that matches the signature.
impl<T> IFunction for T
where
    T: Fn(PtrEnvironment, EvalContext, Vec<PtrValue>) -> PtrValue + Send + Sync,
{
    // #[tracing::instrument(ret, fields(self, env, args), level = "info")]
    fn invoke(&self, env: PtrEnvironment, ctx: EvalContext, args: Vec<PtrValue>) -> PtrValue {
        (self)(env, ctx, args)
    }
}

/// Represents a multi-arity function with multiple implementation bodies.
///
/// Functions can have different implementations for different arities (number of arguments).
/// The function will select the appropriate implementation based on the number of arguments
/// provided at invocation time.
///
/// # Builder Pattern
///
/// Functions are built using the builder pattern for type safety and flexibility.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let func = build_function("my-fn", vec![
///     closure_fn(FunctionArity::Exactly(1), |_env, _ctx, args: Vec<PtrValue>| args[0].clone()),
/// ]);
/// assert_eq!(func.name(), Some("my-fn"));
/// ```
#[derive(Clone)]
pub struct Function {
    /// Optional name for the function (useful for debugging and error messages)
    name: Option<String>,
    /// List of function bodies for different arities
    bodies: Vec<FunctionBody>,
}

impl IFunction for Function {
    // #[tracing::instrument(ret, fields(self, env, args), level = "info")]
    fn invoke(&self, env: PtrEnvironment, ctx: EvalContext, args: Vec<PtrValue>) -> PtrValue {
        let body = self.body_supporting_arg_count(args.len()).expect(&format!(
            "function '{}' invoked with unsupported argument count of {}, expected one of {}; {}",
            self.name.as_ref().unwrap_or(&String::from("<unnamed>")),
            args.len(),
            self.arity_strings().join(","),
            Value::list_ptr(List::from(args.clone())),
        ));
        body.invoke(env, ctx, args)
    }
}

impl IFunction for PtrFunction {
    // #[tracing::instrument(ret, fields(self, env, args), level = "info")]
    fn invoke(&self, env: PtrEnvironment, ctx: EvalContext, args: Vec<PtrValue>) -> PtrValue {
        self.as_ref().invoke(env, ctx, args)
    }
}

impl IHandle for Function {}
impl IHandle for PtrFunction {}

/// A single function implementation with a specific arity.
///
/// Each `FunctionBody` represents one implementation of a function that handles
/// a specific number of arguments. The function will select the appropriate body
/// based on the argument count at invocation time.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let func = build_function(
///     "add",
///     vec![
///         closure_fn(FunctionArity::Exactly(2), |_env, _ctx, args: Vec<PtrValue>| {
///             match (&*args[0], &*args[1]) {
///                 (Value::Integer(a, _), Value::Integer(b, _)) => Value::integer_ptr(a + b),
///                 _ => panic!("expected integers"),
///             }
///         }),
///     ],
/// );
/// assert_eq!(func.name(), Some("add"));
/// ```
#[derive(Clone)]
pub struct FunctionBody {
    /// The arity this body supports
    arity: FunctionArity,
    /// The function implementation
    func: PtrDynIFunction,
}

impl FunctionBody {
    /// Creates a new FunctionBody with the specified arity and implementation.
    ///
    /// # Arguments
    ///
    /// * `arity` - The arity this body supports
    /// * `func` - The function implementation
    pub fn new(arity: FunctionArity, func: PtrDynIFunction) -> Self {
        Self { arity, func }
    }
}

/// Alias for `Arc<dyn IFunction>` - a reference-counted trait object for IFunction implementations.
pub type PtrDynIFunction = Arc<dyn IFunction>;

/// Alias for `Arc<Function>` - a reference-counted pointer to a Function.
pub type PtrFunction = Arc<Function>;

impl Function {
    pub fn builder() -> FunctionBuilder {
        FunctionBuilder::new()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body_supporting_arg_count(&self, arg_count: usize) -> Option<PtrDynIFunction> {
        self.bodies
            .iter()
            .find(|body| body.arity.supports_arg_count(arg_count))
            .map(|body| body.func.clone())
    }

    /// Return a vector of human-readable arity representations.
    /// Example: ["0", "2", "4+"] for Exactly(0), Exactly(2), AtLeast(4).
    pub fn arity_strings(&self) -> Vec<String> {
        self.bodies
            .iter()
            .map(|body| match &body.arity {
                FunctionArity::Exactly(n) => format!("{}", n),
                FunctionArity::AtLeast(n) => format!("{}+", n),
            })
            .collect()
    }
}

/// Represents the arity (number of arguments) a function supports.
///
/// Functions can have multiple arities, allowing them to handle different
/// argument counts. The function will select the appropriate implementation
/// based on the argument count at invocation time.
///
/// # Variants
///
/// - `Exactly(n)` - The function requires exactly n arguments
/// - `AtLeast(n)` - The function requires at least n arguments
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let arity = FunctionArity::Exactly(2);
/// assert!(arity.supports_arg_count(2));
/// assert!(!arity.supports_arg_count(3));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionArity {
    /// The function requires exactly n arguments
    Exactly(usize),
    /// The function requires at least n arguments
    AtLeast(usize),
}

impl FunctionArity {
    /// Returns `true` if this arity supports the given argument count.
    pub fn supports_arg_count(&self, arg_count: usize) -> bool {
        match self {
            Self::Exactly(x) => arg_count == *x,
            Self::AtLeast(x) => arg_count >= *x,
        }
    }
}

/// Builder for constructing `Function` instances.
///
/// The builder provides a fluent API for creating functions with multiple
/// arities and optional names.
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let func = build_function(
///     "my-fn",
///     vec![
///         closure_fn(FunctionArity::Exactly(1), |_env, _ctx, args: Vec<PtrValue>| args[0].clone()),
///         closure_fn(FunctionArity::Exactly(2), |_env, _ctx, args: Vec<PtrValue>| {
///             match (&*args[0], &*args[1]) {
///                 (Value::Integer(a, _), Value::Integer(b, _)) => Value::integer_ptr(a + b),
///                 _ => panic!("expected integers"),
///             }
///         }),
///     ],
/// );
/// assert_eq!(func.name(), Some("my-fn"));
/// ```
pub struct FunctionBuilder {
    /// Optional name for the function
    name: Option<String>,
    /// List of (arity, implementation) pairs
    bodies: Vec<(FunctionArity, PtrDynIFunction)>,
}

impl FunctionBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            bodies: vec![],
        }
    }

    pub fn unset_name(&mut self) -> &mut Self {
        self.name = None;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    pub fn add_body<F>(&mut self, arity: FunctionArity, func: F) -> &mut Self
    where
        F: IFunction + 'static,
    {
        let rc_func = Arc::new(func);
        self.bodies.push((arity, rc_func));
        self
    }

    pub fn with_body<F>(self, arity: FunctionArity, func: F) -> Self
    where
        F: IFunction + 'static,
    {
        let rc_func = Arc::new(func);
        let mut new_bodies = self.bodies;
        new_bodies.push((arity, rc_func));
        Self {
            name: self.name,
            bodies: new_bodies,
        }
    }

    /// Add an `IFunction` implementation directly as an `PtrDynIFunction`.
    ///
    /// This method accepts pre-constructed `IFunction` trait objects, enabling
    /// custom implementations with state or specialized dispatch logic.
    pub fn add_ifunction(&mut self, arity: FunctionArity, func: PtrDynIFunction) -> &mut Self {
        self.bodies.push((arity, func));
        self
    }

    /// Add an `IFunction` implementation directly (builder pattern).
    pub fn with_ifunction(self, arity: FunctionArity, func: PtrDynIFunction) -> Self {
        let mut new_bodies = self.bodies;
        new_bodies.push((arity, func));
        Self {
            name: self.name,
            bodies: new_bodies,
        }
    }

    pub fn clear_bodies(&mut self) -> &mut Self {
        self.bodies.clear();
        self
    }

    /// Build the `Function` with a canonical, deterministic ordering of bodies.
    ///
    /// Ordering rules:
    /// - All `Exactly(n)` entries appear before any `AtLeast(n)` entry.
    /// - `Exactly` entries are sorted by ascending `n` (least-specific first).
    /// - `AtLeast` (variadic) entries are sorted by ascending `n` (least-specific first),
    ///   so `Exactly(1)` precedes `Exactly(2)` and `AtLeast(1)` precedes `AtLeast(2)`; at most one variadic is allowed.
    /// Validation:
    /// - At most one `AtLeast` body is allowed; otherwise this method panics.
    /// - If a single `AtLeast(m)` is present, no `Exactly(n)` may have `n >= m`; otherwise this method panics.
    ///
    /// This keeps dispatch (which picks the first matching body) deterministic and stable
    /// for `Hash` and `PartialEq` which depend on body order.
    pub fn build(self) -> Function {
        let mut exacts: Vec<(FunctionArity, PtrDynIFunction)> = vec![];
        let mut variadics: Vec<(FunctionArity, PtrDynIFunction)> = vec![];

        for (arity, func) in self.bodies {
            match arity {
                FunctionArity::Exactly(n) => exacts.push((FunctionArity::Exactly(n), func)),
                FunctionArity::AtLeast(n) => variadics.push((FunctionArity::AtLeast(n), func)),
            }
        }

        // sort by ascending arity so least-specific handlers come first
        exacts.sort_by(|(a, _), (b, _)| match (a, b) {
            (FunctionArity::Exactly(na), FunctionArity::Exactly(nb)) => na.cmp(nb),
            _ => cmp::Ordering::Equal,
        });

        // variadics should be ordered from least to most: AtLeast(1) before AtLeast(2), etc.
        variadics.sort_by(|(a, _), (b, _)| match (a, b) {
            (FunctionArity::AtLeast(na), FunctionArity::AtLeast(nb)) => na.cmp(nb),
            _ => cmp::Ordering::Equal,
        });

        if variadics.len() > 1 {
            panic!("FunctionBuilder::build: multiple variadic (AtLeast) bodies not supported");
        }

        if let Some((FunctionArity::AtLeast(max_v), _)) = variadics.get(0) {
            for (arity, _) in &exacts {
                if let FunctionArity::Exactly(n) = arity {
                    if *n >= *max_v {
                        panic!(
                            "FunctionBuilder::build: Exactly arity {} is greater-than or equal-to variadic AtLeast {}",
                            n, max_v
                        );
                    }
                }
            }
        }

        let mut bodies = Vec::<FunctionBody>::with_capacity(exacts.len() + variadics.len());
        bodies.extend(exacts.into_iter().map(|(a, f)| FunctionBody::new(a, f)));
        bodies.extend(variadics.into_iter().map(|(a, f)| FunctionBody::new(a, f)));

        Function {
            name: self.name,
            bodies,
        }
    }
}

impl hash::Hash for Function {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        for body in &self.bodies {
            Arc::as_ptr(&body.func).cast::<()>().hash(state);
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        if self.bodies.len() != other.bodies.len() {
            return false;
        }

        for (self_func, other_func) in iter::zip(
            self.bodies.iter().map(|body| &body.func),
            other.bodies.iter().map(|body| &body.func),
        ) {
            if !Arc::ptr_eq(self_func, other_func) {
                return false;
            }
        }

        true
    }
}

impl Eq for Function {}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function{{name: {:?}, bodies: [", self.name)?;
        let mut first = true;
        for body in &self.bodies {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }
            match body.arity {
                FunctionArity::Exactly(n) => write!(f, "Exactly({})", n)?,
                FunctionArity::AtLeast(n) => write!(f, "AtLeast({})", n)?,
            }
        }
        write!(f, "]}}")
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Function {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // compare names
        match (&self.name, &other.name) {
            (Some(a), Some(b)) => {
                let ord = a.cmp(b);
                if ord != cmp::Ordering::Equal {
                    return ord;
                }
            }
            (None, Some(_)) => return cmp::Ordering::Less,
            (Some(_), None) => return cmp::Ordering::Greater,
            (None, None) => {}
        }

        // compare lengths
        let ord = self.bodies.len().cmp(&other.bodies.len());
        if ord != cmp::Ordering::Equal {
            return ord;
        }

        // compare pointers of bodies in order
        for (a_func, b_func) in iter::zip(
            self.bodies.iter().map(|body| &body.func),
            other.bodies.iter().map(|body| &body.func),
        ) {
            let a_ptr = Arc::as_ptr(a_func).cast::<()>();
            let b_ptr = Arc::as_ptr(b_func).cast::<()>();
            let ord = (a_ptr as usize).cmp(&(b_ptr as usize));
            if ord != cmp::Ordering::Equal {
                return ord;
            }
        }

        cmp::Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::sync::Arc;

    #[allow(dead_code)] // inner value is never used
    struct Dummy(&'static str);

    impl IFunction for Dummy {
        fn invoke(
            &self,
            _env: PtrEnvironment,
            _ctx: EvalContext,
            _args: Vec<PtrValue>,
        ) -> PtrValue {
            unimplemented!()
        }
    }

    fn arities(func: &Function) -> Vec<FunctionArity> {
        func.bodies.iter().map(|body| body.arity.clone()).collect()
    }

    #[test]
    fn exact_preferred_over_variadic() {
        // Exactly(2) is allowed when variadic is AtLeast(3)
        let mut func_builder = Function::builder();
        func_builder.add_body(FunctionArity::AtLeast(3), Dummy("var"));
        func_builder.add_body(FunctionArity::Exactly(2), Dummy("ex2"));
        let func = func_builder.build();

        // ordering: Exactly(2) before AtLeast(3)
        let ar = arities(&func);
        assert_eq!(ar.len(), 2);
        match &ar[0] {
            FunctionArity::Exactly(2) => {}
            _ => panic!("expected Exactly(2) first"),
        }

        // ensure body_supporting_arg_count chooses Exactly(2) for arg_count 2
        let body = func.body_supporting_arg_count(2).expect("should find body");
        let stored = &func.bodies[0].func;
        assert!(Arc::ptr_eq(&body, stored));
    }

    #[test]
    fn more_specific_among_exacts() {
        let mut func_builder = Function::builder();
        func_builder.add_body(FunctionArity::Exactly(2), Dummy("ex2"));
        func_builder.add_body(FunctionArity::Exactly(3), Dummy("ex3"));
        let func = func_builder.build();

        // ordering should be Exactly(2), Exactly(3)
        let func_arities = arities(&func);
        assert_eq!(func_arities.len(), 2);
        match &func_arities[0] {
            FunctionArity::Exactly(2) => {}
            _ => panic!("expected Exactly(2) first"),
        }

        let body = func.body_supporting_arg_count(3).expect("should find body");
        // find the stored body corresponding to Exactly(3)
        let idx = func_arities
            .iter()
            .position(|a| *a == FunctionArity::Exactly(3))
            .expect("Exactly(3) present");
        let stored = &func.bodies[idx].func;
        assert!(Arc::ptr_eq(&body, stored));
    }

    #[test]
    #[should_panic]
    fn multiple_variadics_panics() {
        let mut func_builder = Function::builder();
        func_builder.add_body(FunctionArity::AtLeast(1), Dummy("v1"));
        func_builder.add_body(FunctionArity::AtLeast(3), Dummy("v3"));
        let _ = func_builder.build();
    }

    #[test]
    #[should_panic]
    fn exactly_greater_than_variadic_panics() {
        let mut func_builder = Function::builder();
        func_builder.add_body(FunctionArity::Exactly(4), Dummy("ex4"));
        func_builder.add_body(FunctionArity::AtLeast(3), Dummy("v3"));
        let _ = func_builder.build();
    }

    #[test]
    fn canonical_arity_ordering_independent_of_insertion_order() {
        // Build in one order
        let mut b1 = Function::builder();
        b1.add_body(FunctionArity::Exactly(2), Dummy("ex2"));
        b1.add_body(FunctionArity::Exactly(3), Dummy("ex3"));
        let f1 = b1.build();

        // Build in reverse insertion order
        let mut b2 = Function::builder();
        b2.add_body(FunctionArity::Exactly(3), Dummy("ex3"));
        b2.add_body(FunctionArity::Exactly(2), Dummy("ex2"));
        let f2 = b2.build();

        // The canonicalized arity ordering must be the same regardless of insertion order.
        assert_eq!(arities(&f1), arities(&f2));
    }

    #[test]
    fn arity_strings_rendering() {
        let mut func_builder = Function::builder();
        // add in non-canonical order to ensure builder canonicalizes ordering
        func_builder.add_body(FunctionArity::AtLeast(4), Dummy("v"));
        func_builder.add_body(FunctionArity::Exactly(2), Dummy("ex2"));
        func_builder.add_body(FunctionArity::Exactly(0), Dummy("ex0"));
        let f = func_builder.build();

        let parts = f.arity_strings();
        assert_eq!(
            parts,
            vec!["0".to_string(), "2".to_string(), "4+".to_string()]
        );
    }
}
