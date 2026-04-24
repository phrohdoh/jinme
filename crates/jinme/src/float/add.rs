use crate::prelude::*;
use ::std::ops;

// Add Floats & f64s (lhs owned, rhs owned)
// -----------------------------------------------------------------------------

impl ops::Add<Float> for Float {
    type Output = Float;
    fn add(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) + f64::from(rhs))
    }
}

impl ops::Add<f64> for Float {
    type Output = Float;
    fn add(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) + rhs)
    }
}

impl ops::Add<Float> for f64 {
    type Output = Self;
    fn add(self, rhs: Float) -> Self::Output {
        self + f64::from(rhs)
    }
}

// Add Floats & f64s (lhs owned, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'rhs> ops::Add<&'rhs Float> for Float {
    type Output = Float;
    fn add(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) + f64::from(rhs))
    }
}

impl<'rhs> ops::Add<&'rhs f64> for Float {
    type Output = Float;
    fn add(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) + *rhs)
    }
}

impl<'rhs> ops::Add<&'rhs Float> for f64 {
    type Output = f64;
    fn add(self, rhs: &'rhs Float) -> Self::Output {
        self + f64::from(rhs)
    }
}

// Add Floats & f64s (lhs borrowed, rhs owned)
// -----------------------------------------------------------------------------

impl<'lhs> ops::Add<Float> for &'lhs Float {
    type Output = Float;
    fn add(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) + f64::from(rhs))
    }
}

impl<'lhs> ops::Add<f64> for &'lhs Float {
    type Output = Float;
    fn add(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) + rhs)
    }
}

impl<'lhs> ops::Add<Float> for &'lhs f64 {
    type Output = f64;
    fn add(self, rhs: Float) -> Self::Output {
        *self + f64::from(rhs)
    }
}

// Add Floats & f64s (lhs borrowed, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'lhs, 'rhs> ops::Add<&'rhs Float> for &'lhs Float {
    type Output = Float;
    fn add(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) + f64::from(rhs))
    }
}

impl<'lhs, 'rhs> ops::Add<&'rhs f64> for &'lhs Float {
    type Output = Float;
    fn add(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) + rhs)
    }
}

impl<'lhs, 'rhs> ops::Add<&'rhs Float> for &'lhs f64 {
    type Output = f64;
    fn add(self, rhs: &'rhs Float) -> Self::Output {
        *self + f64::from(rhs)
    }
}
