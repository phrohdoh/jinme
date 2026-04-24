use crate::prelude::*;
use ::std::ops;

// Divide Floats & f64s (lhs owned, rhs owned)
// -----------------------------------------------------------------------------

impl ops::Div<Float> for Float {
    type Output = Float;
    fn div(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) / f64::from(rhs))
    }
}

impl ops::Div<f64> for Float {
    type Output = Float;
    fn div(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) / rhs)
    }
}

impl ops::Div<Float> for f64 {
    type Output = Self;
    fn div(self, rhs: Float) -> Self::Output {
        self / f64::from(rhs)
    }
}

// Divide Floats & f64s (lhs owned, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'rhs> ops::Div<&'rhs Float> for Float {
    type Output = Float;
    fn div(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) / f64::from(rhs))
    }
}

impl<'rhs> ops::Div<&'rhs f64> for Float {
    type Output = Float;
    fn div(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) / *rhs)
    }
}

impl<'rhs> ops::Div<&'rhs Float> for f64 {
    type Output = f64;
    fn div(self, rhs: &'rhs Float) -> Self::Output {
        self / f64::from(rhs)
    }
}

// Divide Floats & f64s (lhs borrowed, rhs owned)
// -----------------------------------------------------------------------------

impl<'lhs> ops::Div<Float> for &'lhs Float {
    type Output = Float;
    fn div(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) / f64::from(rhs))
    }
}

impl<'lhs> ops::Div<f64> for &'lhs Float {
    type Output = Float;
    fn div(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) / rhs)
    }
}

impl<'lhs> ops::Div<Float> for &'lhs f64 {
    type Output = f64;
    fn div(self, rhs: Float) -> Self::Output {
        *self / f64::from(rhs)
    }
}

// Divide Floats & f64s (lhs borrowed, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'lhs, 'rhs> ops::Div<&'rhs Float> for &'lhs Float {
    type Output = Float;
    fn div(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) / f64::from(rhs))
    }
}

impl<'lhs, 'rhs> ops::Div<&'rhs f64> for &'lhs Float {
    type Output = Float;
    fn div(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) / rhs)
    }
}

impl<'lhs, 'rhs> ops::Div<&'rhs Float> for &'lhs f64 {
    type Output = f64;
    fn div(self, rhs: &'rhs Float) -> Self::Output {
        *self / f64::from(rhs)
    }
}
