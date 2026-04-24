//! TODO

// ----- Public API ------------------------------------------------------------

/// Get focus [`F`] from source [`S`], via copying/cloning.
pub trait LensGet<S, F> {
    /// Get focus [`F`] from source `s`, via copying/cloning.
    fn get(&self, s: &S) -> F;
}

/// Create a copy/clone of source [`S`] with focus [`F`] set to a specified value.
pub trait LensSet<S, F> {
    /// Create a copy/clone of source `s` with focus set to [`a`].
    fn set(&self, s: &S, a: F) -> S;
}

/// Set focus [`F`] in source [`S`] via in-place mutation through exclusive access.
pub trait LensSetMut<S, F> {
    /// Set focus to `a` in source `s` via in-place mutation through exclusive access.
    fn set(&self, s: &mut S, a: F);
}

/// Get focus [`F`] from source [`S`], via copying/cloning.
///
/// Create a copy/clone of source [`S`] with focus [`F`] set to a specified value.
pub trait Lens<S, F> {
    /// Get focus [`F`] from source `s`, via copying/cloning.
    fn get(&self, s: &S) -> F;
    /// Create a copy/clone of source `s` with focus set to [`a`].
    fn set(&self, s: &S, a: F) -> S;
}

/// Get focus [`F`] from source [`S`], via copying/cloning.
///
/// Set focus [`F`] in source [`S`] via in-place mutation through exclusive access.
pub trait LensMut<S, F> {
    /// Get focus [`F`] from source `s`, via copying/cloning.
    fn get(&self, s: &S) -> F;
    /// Set focus to `a` in source `s` via in-place mutation through exclusive access.
    fn set(&self, s: &mut S, a: F);
}

// A blanket implementation of `Lens` for all types that implements both `LensGet` and `LensSet`.
impl<S, F, T> Lens<S, F> for T
where
    T: LensGet<S, F> + LensSet<S, F>,
{
    fn get(&self, s: &S) -> F {
        LensGet::get(self, s)
    }
    fn set(&self, s: &S, a: F) -> S {
        LensSet::set(self, s, a)
    }
}

// A blanket implementation of `LensMut` for all types that implements both `LensGet` and `LensSetMut`.
impl<S, F, T> LensMut<S, F> for T
where
    T: LensGet<S, F> + LensSetMut<S, F>,
{
    fn get(&self, s: &S) -> F {
        LensGet::get(self, s)
    }
    fn set(&self, s: &mut S, a: F) {
        LensSetMut::set(self, s, a)
    }
}

// ----- Internal API (concrete implementations) -------------------------------

/// A concrete [`Lens`] implementation using function pointers.
///
/// `S`: Source.
/// `F`: Focus.
/// `O`: Output of the `set` function, which is `S` by default.
pub(crate) struct LensImpl<S, F, O = S> {
    get: fn(&S) -> F,
    set: fn(&S, F) -> O,
}

impl<S, F, O> LensImpl<S, F, O> {
    pub fn new(get: fn(&S) -> F, set: fn(&S, F) -> O) -> Self {
        Self { get, set }
    }
}

impl<S, F> LensGet<S, F> for LensImpl<S, F> {
    fn get(&self, s: &S) -> F {
        (self.get)(s)
    }
}

impl<S, F> LensSet<S, F> for LensImpl<S, F> {
    fn set(&self, s: &S, a: F) -> S {
        (self.set)(s, a)
    }
}

/// A concrete [`LensMut`] implementation using function pointers.
///
/// `S`: Source.
/// `F`: Focus.
pub(crate) struct LensMutImpl<S, F> {
    get: fn(&S) -> F,
    set: fn(&mut S, F),
}

impl<S, F> LensMutImpl<S, F> {
    pub fn new(get: fn(&S) -> F, set: fn(&mut S, F)) -> Self {
        Self { get, set }
    }
}

impl<S, F> LensGet<S, F> for LensMutImpl<S, F> {
    fn get(&self, s: &S) -> F {
        (self.get)(s)
    }
}

impl<S, F> LensSetMut<S, F> for LensMutImpl<S, F> {
    fn set(&self, s: &mut S, a: F) {
        (self.set)(s, a)
    }
}
