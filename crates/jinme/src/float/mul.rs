use crate::prelude::*;
use ::std::ops;

// Multiply Floats & f64s (lhs owned, rhs owned)
// -----------------------------------------------------------------------------

impl ops::Mul<Float> for Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) * f64::from(rhs))
    }
}

impl ops::Mul<f64> for Float {
    type Output = Float;
    fn mul(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) * rhs)
    }
}

impl ops::Mul<Float> for f64 {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        self * f64::from(rhs)
    }
}

// Multiply Floats & f64s (lhs owned, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'rhs> ops::Mul<&'rhs Float> for Float {
    type Output = Float;
    fn mul(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) * f64::from(rhs))
    }
}

impl<'rhs> ops::Mul<&'rhs f64> for Float {
    type Output = Float;
    fn mul(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) * *rhs)
    }
}

impl<'rhs> ops::Mul<&'rhs Float> for f64 {
    type Output = f64;
    fn mul(self, rhs: &'rhs Float) -> Self::Output {
        self * f64::from(rhs)
    }
}

// Multiply Floats & f64s (lhs borrowed, rhs owned)
// -----------------------------------------------------------------------------

impl<'lhs> ops::Mul<Float> for &'lhs Float {
    type Output = Float;
    fn mul(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) * f64::from(rhs))
    }
}

impl<'lhs> ops::Mul<f64> for &'lhs Float {
    type Output = Float;
    fn mul(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) * rhs)
    }
}

impl<'lhs> ops::Mul<Float> for &'lhs f64 {
    type Output = f64;
    fn mul(self, rhs: Float) -> Self::Output {
        *self * f64::from(rhs)
    }
}

// Multiply Floats & f64s (lhs borrowed, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'lhs, 'rhs> ops::Mul<&'rhs Float> for &'lhs Float {
    type Output = Float;
    fn mul(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) * f64::from(rhs))
    }
}

impl<'lhs, 'rhs> ops::Mul<&'rhs f64> for &'lhs Float {
    type Output = Float;
    fn mul(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) * rhs)
    }
}

impl<'lhs, 'rhs> ops::Mul<&'rhs Float> for &'lhs f64 {
    type Output = f64;
    fn mul(self, rhs: &'rhs Float) -> Self::Output {
        *self * f64::from(rhs)
    }
}
