use crate::{prelude::*, value::optics as value_optics};
use std::sync::Arc;

/// Evaluates a value in the given environment and context.
///
/// This is the main evaluation function that handles different value types:
/// - `nil` and literals are returned as-is
/// - Symbols are resolved to their bound values
/// - Lists are evaluated as function calls or special forms
/// - Vectors, sets, and maps are recursively evaluated element-wise
/// - Vars are dereferenced to their bound values
/// - Functions and handles are returned as-is
///
/// # Arguments
///
/// * `env` - The environment to use for symbol resolution
/// * `ctx` - The evaluation context (for local bindings)
/// * `v` - The value to evaluate
///
/// # Returns
///
/// The evaluated value as a `PtrValue`
///
/// # Special Forms
///
/// Lists starting with certain symbols are treated as special forms:
/// - `let*` - Sequential binding with context threading
/// - `fn*` - Function definition
/// - `do` - Sequential expression evaluation
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let mut builder = Environment::builder();
/// builder.insert_namespace(Arc::new(Namespace::new_empty("clojure.core")));
/// builder.set_current_namespace_var("clojure.core", "*ns*");
/// let env = builder.build_ptr();
/// let ctx = EvalContext::new_empty();
/// let result = eval(env, ctx, Value::integer_ptr(42));
/// assert_eq!(result, Value::integer_ptr(42));
/// ```
pub fn eval(env: PtrEnvironment, ctx: EvalContext, v: PtrValue) -> PtrValue {
    match v.as_ref() {
        Value::Nil(_) => v,
        Value::Symbol(symbol, _) => {
            // Check local bindings first (from let*, fn* parameters, etc.)
            if let Symbol::Unqualified(sym_unq) = symbol {
                if let Some(local_value) = ctx.resolve_local(sym_unq.name()) {
                    return local_value;
                }
            }
            // Fall back to namespace resolution
            resolve_or_panic(env.clone(), symbol)
                .deref()
                .expect(&format!("attempted to deref unbound Var: {}", symbol))
        }
        Value::Keyword(_, _) => v,
        Value::Boolean(_, _) => v,
        Value::Integer(_, _) => v,
        Value::Float(_, _) => v,
        Value::String(_, _) => v,
        Value::List(list, _) => {
            if list.is_empty() {
                return v;
            }

            // Check for special forms before evaluating arguments
            if let Some(head) = list.get_first() {
                if let Value::Symbol(Symbol::Unqualified(head_sym), _) = head.as_ref() {
                    match head_sym.name() {
                        "let*" => return eval_let_star(env.clone(), ctx.clone(), list),
                        "fn*" => return eval_fn_star(env.clone(), ctx.clone(), list),
                        "do" => return eval_do(env.clone(), ctx.clone(), list),
                        _ => {}
                    }
                }
            }

            // Regular function application
            let args: Vec<PtrValue> = list
                .iter()
                .skip(1)
                .map(|value| eval(env.clone(), ctx.clone(), value.to_owned()))
                .collect();
            let v = eval(
                env.clone(),
                ctx.clone(),
                list.get_first().unwrap().to_owned(),
            );
            apply(env.clone(), ctx.clone(), v, args)
        }
        Value::Vector(vector, _) => Value::new_vector_ptr(
            vector
                .iter()
                .map(|value| eval(env.clone(), ctx.clone(), value.to_owned()))
                .collect(),
        ),
        Value::Set(set, _) => Value::new_set_ptr(
            set.iter()
                .map(|value| eval(env.clone(), ctx.clone(), value.to_owned()))
                .collect(),
        ),
        Value::Map(map, _) => Value::new_map_ptr(
            map.iter()
                .map(|(k, v)| {
                    (
                        eval(env.clone(), ctx.clone(), k.to_owned()),
                        eval(env.clone(), ctx.clone(), v.to_owned()),
                    )
                })
                .collect(),
        ),
        Value::Var(var, _) => var.deref().expect("attempted to deref unbound Var"),
        Value::Function(_, _) => v,
        Value::Handle(_, _) => v,
    }
}

/// Applies a function to a list of arguments.
///
/// This function handles function invocation for different function types:
/// - `Function` values invoke their bodies with the provided arguments
/// - `Handle` values are downcast to `Function` if possible
/// - Other types return the function value unchanged (with a warning)
///
/// # Arguments
///
/// * `env` - The environment to use
/// * `ctx` - The evaluation context
/// * `f` - The function value to apply
/// * `args` - The arguments to pass to the function
///
/// # Returns
///
/// The result of the function application
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// # use std::sync::Arc;
/// let mut builder = Environment::builder();
/// builder.insert_namespace(Arc::new(Namespace::new_empty("clojure.core")));
/// builder.set_current_namespace_var("clojure.core", "*ns*");
/// let env = builder.build_ptr();
/// let ctx = EvalContext::new_empty();
/// let func = build_function_value_ptr("add", vec![
///     closure_fn(FunctionArity::Exactly(2), |_env, _ctx, args: Vec<PtrValue>| {
///         match (&*args[0], &*args[1]) {
///             (Value::Integer(a, _), Value::Integer(b, _)) => Value::integer_ptr(a + b),
///             _ => panic!("expected integers"),
///         }
///     }),
/// ]);
/// let result = apply(env, ctx, func, vec![Value::integer_ptr(1), Value::integer_ptr(2)]);
/// assert_eq!(result, Value::integer_ptr(3));
/// ```
pub fn apply(env: PtrEnvironment, ctx: EvalContext, f: PtrValue, args: Vec<PtrValue>) -> PtrValue {
    match f.as_ref() {
        Value::Function(func, _) => func.invoke(env.clone(), ctx.clone(), args),
        Value::Handle(handle, _) => {
            if let Some(func) = handle.downcast_ref::<Function>() {
                func.invoke(env.clone(), ctx.clone(), args)
            } else {
                f
            }
        }
        // TODO: properly handle other variants
        _ => {
            eprintln!("Warning: apply called on non-function value: {:?}", f);
            f
        }
    }
}

/// Special form: do
/// (do expr1 expr2 ... exprN) -> evaluate each in order, return value of exprN (or nil if empty)
fn eval_do(env: PtrEnvironment, ctx: EvalContext, list: &List) -> PtrValue {
    let body_exprs: Vec<PtrValue> = list.iter().skip(1).map(|v| v.to_owned()).collect();

    if body_exprs.is_empty() {
        return Value::nil_ptr();
    }

    let mut result = Value::nil_ptr();
    for expr in body_exprs {
        result = eval(env.clone(), ctx.clone(), expr);
    }
    result
}

/// Special form: let*
/// (let* [binding1 expr1 binding2 expr2 ...] body_expr1 body_expr2 ...)
/// Binds variables sequentially, later bindings can see earlier ones.
fn eval_let_star(env: PtrEnvironment, ctx: EvalContext, list: &List) -> PtrValue {
    // Extract binding vector and body
    if let Some(binding_vec) = list.get_second() {
        if let Value::Vector(bindings, _) = binding_vec.as_ref() {
            // Validate even number of elements
            if bindings.len() % 2 != 0 {
                panic!("let*: binding vector must have an even number of elements");
            }

            // Process bindings sequentially with context threading
            let body_exprs: Vec<PtrValue> = list.iter().skip(2).map(|v| v.to_owned()).collect();

            if body_exprs.is_empty() {
                return Value::nil_ptr();
            }

            // Helper function to process bindings recursively
            fn process_bindings(
                env: PtrEnvironment,
                ctx: EvalContext,
                bindings: &Vector,
                binding_index: usize,
                body_exprs: &[PtrValue],
            ) -> PtrValue {
                if binding_index >= bindings.len() {
                    // All bindings processed, evaluate body
                    let mut res = Value::nil_ptr();
                    for expr in body_exprs {
                        res = eval(env.clone(), ctx.clone(), expr.to_owned());
                    }
                    return res;
                }

                // Get next binding pair
                let var_name = &bindings.get_nth(binding_index).expect("let* binding name");
                let init_expr = &bindings
                    .get_nth(binding_index + 1)
                    .expect("let* binding init");

                // Evaluate init expression with current context
                let value = eval(env.clone(), ctx.clone(), init_expr.to_owned());

                // Extract symbol name and extend context
                if let Some(_sym) = value_optics::preview_symbol_unqualified(var_name) {
                    todo!()
                }
                if let Value::Symbol(Symbol::Unqualified(sym_unq), _) = var_name.as_ref() {
                    let var_name_str = sym_unq.name().to_string();
                    let new_ctx = ctx.with_local(var_name_str, value);

                    // Continue processing remaining bindings with extended context
                    process_bindings(
                        env.clone(),
                        new_ctx,
                        bindings,
                        binding_index + 2,
                        body_exprs,
                    )
                } else {
                    panic!("let*: binding name must be a symbol");
                }
            }

            return process_bindings(env, ctx, bindings, 0, &body_exprs);
        }
    }

    panic!("let*: first argument must be a binding vector");
}

/// Special form: fn*
/// (fn* [param1 param2 ...] body_expr1 body_expr2 ...)
/// (fn* ([param1] body) ([param1 param2] body2) ...) for multiple arities
/// Creates a closure that captures the current evaluation context.
fn eval_fn_star(env: PtrEnvironment, ctx: EvalContext, list: &List) -> PtrValue {
    // Capture the current context at function definition time
    let captured_ctx = ctx;

    // Check if second element is a vector (single arity) or list (multi-arity)
    if let Some(second) = list.get_second() {
        match second.as_ref() {
            Value::Vector(params_vec, _) => {
                // Single-arity fn*
                let params = extract_params(params_vec);
                let body: Vec<PtrValue> = list.iter().skip(2).map(|v| v.to_owned()).collect();
                return create_fn_closure(env, captured_ctx, vec![(params, body)]);
            }
            Value::List(_, _) => {
                // Multi-arity fn*
                let mut arities = vec![];
                let forms: Vec<PtrValue> = list.iter().skip(1).map(|v| v.to_owned()).collect();

                for form in forms {
                    if let Value::List(form_list, _) = form.as_ref() {
                        if let Some(param_vec) = form_list.get_first() {
                            if let Value::Vector(params_vec, _) = param_vec.as_ref() {
                                let params = extract_params(params_vec);
                                let body: Vec<PtrValue> =
                                    form_list.iter().skip(1).map(|v| v.to_owned()).collect();
                                arities.push((params, body));
                            } else {
                                panic!("fn*: multi-arity form parameters must be a vector");
                            }
                        }
                    } else {
                        panic!("fn*: multi-arity forms must be lists");
                    }
                }

                return create_fn_closure(env, captured_ctx, arities);
            }
            _ => {
                panic!(
                    "fn*: first argument must be a parameter vector or list of parameter vectors"
                );
            }
        }
    }

    panic!("fn*: missing parameters");
}

/// Extract parameter names from a parameter vector.
/// Supports variadic syntax: [a b & rest] -> (["a", "b"], Some("rest"))
fn extract_params(params_vec: &Vector) -> FnParams {
    let mut regular_params = vec![]; // TODO: support destructuring
    let mut variadic_param: Option<String> = None; // TODO: support destructuring
    let mut saw_ampersand = false;

    for param_val in params_vec.iter() {
        if let Value::Symbol(Symbol::Unqualified(sym), _) = param_val.as_ref() {
            if sym.name() == "&" {
                saw_ampersand = true;
            } else if saw_ampersand {
                variadic_param = Some(sym.name().to_string());
                saw_ampersand = false;
            } else {
                regular_params.push(sym.name().to_string());
            }
        } else {
            panic!("fn*: parameter must be a symbol");
        }
    }

    FnParams {
        regular: regular_params,
        variadic: variadic_param,
    }
}

#[derive(Clone, Debug)]
struct FnParams {
    regular: Vec<String>,
    variadic: Option<String>,
}

fn create_fn_closure(
    env: PtrEnvironment,
    captured_ctx: EvalContext,
    arities: Vec<(FnParams, Vec<PtrValue>)>,
) -> PtrValue {
    let mut func_builder = Function::builder();

    for (params, body) in arities {
        let arity = if let Some(ref _variadic) = params.variadic {
            FunctionArity::AtLeast(params.regular.len())
        } else {
            FunctionArity::Exactly(params.regular.len())
        };

        let _env_clone = env.clone();
        let params_clone = params.clone();
        let body_clone = body.clone();
        let ctx_clone = captured_ctx.clone();
        let arity_clone = arity.clone();

        let closure = closure_fn(
            arity,
            move |fn_env: PtrEnvironment, _fn_ctx: EvalContext, args: Vec<PtrValue>| {
                // Bind parameters to arguments
                let mut fn_ctx = ctx_clone.clone();

                // Bind regular parameters
                for (i, param_name) in params_clone.regular.iter().enumerate() {
                    if i < args.len() {
                        fn_ctx = fn_ctx.with_local(param_name.clone(), args[i].clone());
                    } else {
                        panic!("fn*: not enough arguments");
                    }
                }

                // Bind variadic parameter if present
                if let Some(ref variadic_name) = params_clone.variadic {
                    let rest_args = args[params_clone.regular.len()..].to_vec();
                    let rest_list = Value::list_ptr(List::from(rest_args));
                    fn_ctx = fn_ctx.with_local(variadic_name.clone(), rest_list);
                } else if args.len() > params_clone.regular.len() {
                    panic!("fn*: too many arguments");
                }

                // Evaluate body with extended context
                let mut res = Value::nil_ptr();
                for expr in &body_clone {
                    res = eval(fn_env.clone(), fn_ctx.clone(), expr.to_owned());
                }
                res
            },
        );

        func_builder.add_ifunction(arity_clone, closure.1);
    }

    Value::function_ptr(Arc::new(func_builder.build()))
}

pub fn try_resolve(env: PtrEnvironment, symbol: &Symbol) -> Result<PtrVar, ResolveError> {
    match symbol {
        Symbol::Qualified(sym) => {
            // log::info!("Resolving qualified symbol: {}", sym);
            env.try_get_namespace(sym.namespace())
                .ok_or_else(|| {
                    ResolveError::NoSuchNamespace(SymbolUnqualified::new(sym.namespace()))
                })?
                .try_get_var(sym.name())
                .map_err(ResolveError::from)
        }
        Symbol::Unqualified(sym) => {
            // log::info!("Resolving unqualified symbol: {}", sym);
            env.try_get_current_namespace()
                .map_err(|_| ResolveError::UnknownCurrentNamespace)?
                .try_get_var(sym.name())
                .map_err(ResolveError::from)
        }
    }
}

pub fn resolve_or_panic(env: PtrEnvironment, symbol: &Symbol) -> PtrVar {
    match symbol {
        Symbol::Qualified(sym) => {
            // log::info!("Resolving qualified symbol: {}", sym);
            env.try_get_namespace(sym.namespace())
                .ok_or_else(|| {
                    ResolveError::NoSuchNamespace(SymbolUnqualified::new(sym.namespace()))
                })
                .expect(&format!("could not find namespace: {}", sym.namespace()))
                .try_get_var(sym.name())
                .expect(&format!("could not resolve var: {}", sym))
        }
        Symbol::Unqualified(sym) => {
            // log::info!("Resolving unqualified symbol: {}", sym);
            env.try_get_current_namespace()
                .map_err(|_| ResolveError::UnknownCurrentNamespace)
                .expect(&format!("could not determine current namespace"))
                .try_get_var(sym.name())
                .expect(&format!("could not resolve var: {}", sym))
        }
    }
}

#[derive(Debug, Clone)]
pub enum ResolveError {
    NoSuchNamespace(SymbolUnqualified),
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    UnknownCurrentNamespace,
}

impl From<GetVarError> for ResolveError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetValueError> for ResolveError {
    fn from(get_value_err: GetValueError) -> Self {
        match get_value_err {
            GetValueError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
            GetValueError::UnboundVar(var_sym) => Self::UnboundVar(var_sym),
        }
    }
}
