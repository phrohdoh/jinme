//! TODO

use ::std::sync::Arc;

// ----- Public API ------------------------------------------------------------

/// A trait for extracting a value from a source by cloning/copying.
pub trait PrismPreview<S, A> {
    /// Extracts `A` from the source if it matches the expected variant.
    /// Returns `None` if the source doesn't match.
    fn preview(&self, source: &S) -> Option<A>;
}

/// A trait for constructing a source from a value.
pub trait PrismReview<S, A> {
    /// Constructs a source from a value of type `A`.
    fn review(&self, a: A) -> Arc<S>;
}

/// A trait for modifying an extracted value within a source.
pub trait PrismModify<S, A>: PrismPreview<S, A> {
    /// Applies a function to the extracted value, if the source matches.
    /// Returns a new source with the modified value, or the original if it doesn't match.
    fn modify<F>(&self, source: Arc<S>, f: F) -> Arc<S>
    where
        F: Fn(A) -> A;
}

/// A trait for setting a value within a source.
pub trait PrismSet<S, A>: PrismModify<S, A> {
    /// Sets the value in the source, returning a new source.
    /// Returns a new source with the provided value if the source matches,
    /// or the original if it doesn't match.
    fn set(&self, source: Arc<S>, a: A) -> Arc<S>
    where
        A: Clone;
}

/// A trait for attempting to modify a value with error handling.
pub trait PrismTryModify<S, A>: PrismPreview<S, A> {
    /// Attempts to apply a function to the extracted value.
    /// Returns `Ok(modified_value)` if the source matches, `Err(original_value)` otherwise.
    fn try_modify<F>(&self, source: Arc<S>, f: F) -> Result<Arc<S>, Arc<S>>
    where
        F: Fn(A) -> A;
}

/// A marker trait that combines all prism operations.
pub trait Prism<S, A>:
    PrismPreview<S, A> + PrismReview<S, A> + PrismModify<S, A> + PrismSet<S, A> + PrismTryModify<S, A>
{
}

impl<T, S, A> Prism<S, A> for T where
    T: PrismPreview<S, A>
        + PrismReview<S, A>
        + PrismModify<S, A>
        + PrismSet<S, A>
        + PrismTryModify<S, A>
{
}

// ----- Internal API (concrete implementation) --------------------------------

/// A concrete prism implementation using function pointers.
// #[derive(Clone)]
pub(crate) struct PrismImpl<S, A> {
    preview: fn(&S) -> Option<A>,
    review: fn(A) -> Arc<S>,
}

impl<S, A> PrismImpl<S, A> {
    pub fn new(preview: fn(&S) -> Option<A>, review: fn(A) -> Arc<S>) -> Self {
        Self { preview, review }
    }
}

impl<S, A> PrismPreview<S, A> for PrismImpl<S, A> {
    fn preview(&self, source: &S) -> Option<A> {
        (self.preview)(source)
    }
}

impl<S, A> PrismReview<S, A> for PrismImpl<S, A> {
    fn review(&self, a: A) -> Arc<S> {
        (self.review)(a)
    }
}

impl<S, A> PrismModify<S, A> for PrismImpl<S, A> {
    fn modify<F>(&self, source: Arc<S>, f: F) -> Arc<S>
    where
        F: Fn(A) -> A,
    {
        match self.preview(source.as_ref()) {
            Some(a) => self.review(f(a)),
            None => source,
        }
    }
}

impl<S, A> PrismSet<S, A> for PrismImpl<S, A>
where
    A: Clone,
{
    fn set(&self, source: Arc<S>, a: A) -> Arc<S> {
        self.modify(source, |_| a.clone())
    }
}

impl<S, A> PrismTryModify<S, A> for PrismImpl<S, A> {
    fn try_modify<F>(&self, source: Arc<S>, f: F) -> Result<Arc<S>, Arc<S>>
    where
        F: Fn(A) -> A,
    {
        match self.preview(source.as_ref()) {
            Some(a) => Ok(self.review(f(a))),
            None => Err(source),
        }
    }
}
