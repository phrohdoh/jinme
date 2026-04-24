//! TODO

use crate::prelude::*;
use ::std::sync::Arc;

// ----- Public API ------------------------------------------------------------

pub trait Prism<S, F>:
    PrismPreview<S, F> + PrismReview<S, F> + PrismModify<S, F> + PrismSet<S, F> + PrismTryModify<S, F>
{
}

impl<S, F, T> Prism<S, F> for T where
    T: PrismPreview<S, F>
        + PrismReview<S, F>
        + PrismModify<S, F>
        + PrismSet<S, F>
        + PrismTryModify<S, F>
{
}

/// Get focus [`F`] from source [`S`], via copying/cloning.
pub trait PrismPreview<S, F> {
    /// Get focus [`F`] from source `s`, via copying/cloning.
    fn preview(&self, source: &S) -> Option<F>;
}

pub trait PrismReview<S, F> {
    fn review(&self, a: F) -> Arc<S>;
}

pub trait PrismModify<S, F> {
    fn modify<M>(&self, source: Arc<S>, func: M) -> Arc<S>
    where
        M: Fn(F) -> F;
}

pub trait PrismSet<S, F> {
    fn set(&self, source: Arc<S>, a: F) -> Arc<S>
    where
        F: Clone;
}

pub trait PrismTryModify<S, F> {
    fn try_modify<M>(&self, source: Arc<S>, f: M) -> Result<Arc<S>, Arc<S>>
    where
        M: Fn(F) -> F;
}

// ----- Internal API (concrete implementations) -------------------------------

pub(crate) struct PrismImpl<S, F> {
    preview: fn(&S) -> Option<F>,
    review: fn(F) -> Arc<S>,
}

impl<S, F> PrismImpl<S, F> {
    pub fn new(preview: fn(&S) -> Option<F>, review: fn(F) -> Arc<S>) -> Self {
        Self { preview, review }
    }
}

impl<S, F> PrismPreview<S, F> for PrismImpl<S, F> {
    fn preview(&self, source: &S) -> Option<F> {
        (self.preview)(source)
    }
}

impl<S, F> PrismReview<S, F> for PrismImpl<S, F> {
    fn review(&self, a: F) -> Arc<S> {
        (self.review)(a)
    }
}

impl<S, F> PrismModify<S, F> for PrismImpl<S, F> {
    fn modify<M>(&self, source: Arc<S>, f: M) -> Arc<S>
    where
        M: Fn(F) -> F,
    {
        match self.preview(source.as_ref()) {
            Some(a) => self.review(f(a)),
            None => source,
        }
    }
}

impl<S, F> PrismSet<S, F> for PrismImpl<S, F>
where
    F: Clone,
{
    fn set(&self, source: Arc<S>, a: F) -> Arc<S> {
        self.modify(source, |_| a.clone())
    }
}

impl<S, F> PrismTryModify<S, F> for PrismImpl<S, F> {
    fn try_modify<M>(&self, source: Arc<S>, f: M) -> Result<Arc<S>, Arc<S>>
    where
        M: Fn(F) -> F,
    {
        match self.preview(source.as_ref()) {
            Some(a) => Ok(self.review(f(a))),
            None => Err(source),
        }
    }
}

pub fn prism_nil() -> impl Prism<Value, ()> {
    PrismImpl::new(
        |source| match source {
            Value::Nil(_) => Some(()),
            _ => None,
        },
        |_| Value::nil_ptr(),
    )
}

pub fn prism_integer() -> impl Prism<Value, i64> {
    PrismImpl::new(
        |source| match source {
            Value::Integer(i, _) => Some(*i),
            _ => None,
        },
        |i| Value::integer_ptr(i),
    )
}

pub fn prism_float() -> impl Prism<Value, f64> {
    PrismImpl::new(
        |source| match source {
            Value::Float(fl, _) => Some(fl.as_f64()),
            _ => None,
        },
        |f| Value::float_ptr(f.into()),
    )
}

pub fn prism_string() -> impl Prism<Value, String> {
    PrismImpl::new(
        |source| match source {
            Value::String(s, _) => Some(s.clone()),
            _ => None,
        },
        |s| Value::string_ptr(s),
    )
}

pub fn prism_symbol() -> impl Prism<Value, Symbol> {
    PrismImpl::new(
        |source| match source {
            Value::Symbol(sym, _) => Some(sym.clone()),
            _ => None,
        },
        |sym| Value::symbol_ptr(sym),
    )
}

pub fn prism_keyword() -> impl Prism<Value, Keyword> {
    PrismImpl::new(
        |source| match source {
            Value::Keyword(kw, _) => Some(kw.clone()),
            _ => None,
        },
        |kw| Value::keyword_ptr(kw),
    )
}

pub fn prism_list() -> impl Prism<Value, List> {
    PrismImpl::new(
        |source| match source {
            Value::List(list, _) => Some(list.clone()),
            _ => None,
        },
        |list| Value::list_ptr(list),
    )
}

pub fn prism_vector() -> impl Prism<Value, Vector> {
    PrismImpl::new(
        |source| match source {
            Value::Vector(vector, _) => Some(vector.clone()),
            _ => None,
        },
        |vector| Value::vector_ptr(vector),
    )
}

pub fn prism_set() -> impl Prism<Value, Set> {
    PrismImpl::new(
        |source| match source {
            Value::Set(set, _) => Some(set.clone()),
            _ => None,
        },
        |set| Value::set_ptr(set),
    )
}

pub fn prism_map() -> impl Prism<Value, Map> {
    PrismImpl::new(
        |source| match source {
            Value::Map(map, _) => Some(map.clone()),
            _ => None,
        },
        |map| Value::map_ptr(map),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn prism_nil() {
        let prism_nil = super::prism_nil();
        assert_eq!(prism_nil.preview(&Value::nil()), Some(()));
        assert_eq!(prism_nil.preview(&Value::integer(42)), None);
        assert_eq!(*prism_nil.review(()), Value::nil());
    }

    #[test]
    fn prism_integer() {
        let prism_integer = super::prism_integer();
        assert_eq!(prism_integer.preview(&Value::integer(42)), Some(42));
        assert_eq!(prism_integer.preview(&Value::nil()), None);
        assert_eq!(*prism_integer.review(42), Value::integer(42));
    }

    #[test]
    fn prism_float() {
        let prism_float = super::prism_float();
        assert_eq!(prism_float.preview(&Value::float(3.14.into())), Some(3.14));
        assert_eq!(prism_float.preview(&Value::nil()), None);
        assert_eq!(*prism_float.review(3.14), Value::float(3.14.into()));
    }

    #[test]
    fn prism_string() {
        let prism_string = super::prism_string();
        assert_eq!(
            prism_string.preview(&Value::string("hello".into())),
            Some("hello".to_string())
        );
        assert_eq!(prism_string.preview(&Value::nil()), None);
        assert_eq!(
            *prism_string.review("hello".to_string()),
            Value::string("hello".into())
        );
    }

    #[test]
    fn prism_symbol() {
        let prism_symbol = super::prism_symbol();
        assert_eq!(
            prism_symbol.preview(&Value::symbol_unqualified("sym")),
            Some(Symbol::new_unqualified("sym"))
        );
        assert_eq!(prism_symbol.preview(&Value::nil()), None);
        assert_eq!(
            *prism_symbol.review(Symbol::new_unqualified("sym")),
            Value::symbol_unqualified("sym")
        );
    }

    #[test]
    fn prism_keyword() {
        let prism_keyword = super::prism_keyword();
        assert_eq!(
            prism_keyword.preview(&Value::keyword_unqualified("kw")),
            Some(Keyword::new_unqualified("kw"))
        );
        assert_eq!(prism_keyword.preview(&Value::nil()), None);
        assert_eq!(
            *prism_keyword.review(Keyword::new_unqualified("kw")),
            Value::keyword_unqualified("kw")
        );
    }

    #[test]
    fn prism_list() {
        let prism_list = super::prism_list();
        assert_eq!(
            prism_list.preview(&Value::list_from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ])),
            Some(List::from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ]))
        );
        assert_eq!(prism_list.preview(&Value::nil()), None);
        assert_eq!(
            *prism_list.review(List::from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ])),
            Value::list_from(vec![Value::integer_ptr(1), Value::integer_ptr(2)])
        );
    }

    #[test]
    fn prism_vector() {
        let prism_vector = super::prism_vector();
        assert_eq!(
            prism_vector.preview(&Value::vector_from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ])),
            Some(Vector::from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ]))
        );
        assert_eq!(prism_vector.preview(&Value::nil()), None);
        assert_eq!(
            *prism_vector.review(Vector::from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ])),
            Value::vector_from(vec![Value::integer_ptr(1), Value::integer_ptr(2)])
        );
    }

    #[test]
    fn prism_set() {
        let prism_set = super::prism_set();
        assert_eq!(
            prism_set.preview(&Value::set_from(vec![
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            ])),
            Some(Set::new(vec![Value::integer_ptr(1), Value::integer_ptr(2)]))
        );
        assert_eq!(prism_set.preview(&Value::nil()), None);
        assert_eq!(
            *prism_set.review(Set::new(vec![Value::integer_ptr(1), Value::integer_ptr(2)])),
            Value::set_from(vec![Value::integer_ptr(1), Value::integer_ptr(2)])
        );
    }

    #[test]
    fn prism_map() {
        let prism_map = super::prism_map();
        let test_map = Map::new(vec![(Value::integer_ptr(1), Value::integer_ptr(2))]);
        assert_eq!(
            prism_map.preview(&Value::map_from(vec![(
                Value::integer_ptr(1),
                Value::integer_ptr(2)
            )])),
            Some(test_map.clone())
        );
        assert_eq!(prism_map.preview(&Value::nil()), None);
        let test_map2 = Map::new(vec![(Value::integer_ptr(1), Value::integer_ptr(2))]);
        assert_eq!(
            *prism_map.review(test_map2),
            Value::map_from(vec![(Value::integer_ptr(1), Value::integer_ptr(2))])
        );
    }

    #[test]
    #[ignore]
    fn prism_var() {
        todo!("prism_var")
    }

    #[test]
    #[ignore]
    fn prism_function() {
        todo!("prism_function")
    }

    #[test]
    #[ignore]
    fn prism_handle() {
        todo!("prism_handle")
    }
}
