use crate::prelude::*;
use ::std::ops;

// Sub Floats & f64s (lhs owned, rhs owned)
// -----------------------------------------------------------------------------

impl ops::Sub<Float> for Float {
    type Output = Float;
    fn sub(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) - f64::from(rhs))
    }
}

impl ops::Sub<f64> for Float {
    type Output = Float;
    fn sub(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) - rhs)
    }
}

impl ops::Sub<Float> for f64 {
    type Output = Self;
    fn sub(self, rhs: Float) -> Self::Output {
        self - f64::from(rhs)
    }
}

// Sub Floats & f64s (lhs owned, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'rhs> ops::Sub<&'rhs Float> for Float {
    type Output = Float;
    fn sub(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) - f64::from(rhs))
    }
}

impl<'rhs> ops::Sub<&'rhs f64> for Float {
    type Output = Float;
    fn sub(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) - *rhs)
    }
}

impl<'rhs> ops::Sub<&'rhs Float> for f64 {
    type Output = f64;
    fn sub(self, rhs: &'rhs Float) -> Self::Output {
        self - f64::from(rhs)
    }
}

// Sub Floats & f64s (lhs borrowed, rhs owned)
// -----------------------------------------------------------------------------

impl<'lhs> ops::Sub<Float> for &'lhs Float {
    type Output = Float;
    fn sub(self, rhs: Float) -> Self::Output {
        Float::from(f64::from(self) - f64::from(rhs))
    }
}

impl<'lhs> ops::Sub<f64> for &'lhs Float {
    type Output = Float;
    fn sub(self, rhs: f64) -> Self::Output {
        Float::from(f64::from(self) - rhs)
    }
}

impl<'lhs> ops::Sub<Float> for &'lhs f64 {
    type Output = f64;
    fn sub(self, rhs: Float) -> Self::Output {
        *self - f64::from(rhs)
    }
}

// Sub Floats & f64s (lhs borrowed, rhs borrowed)
// -----------------------------------------------------------------------------

impl<'lhs, 'rhs> ops::Sub<&'rhs Float> for &'lhs Float {
    type Output = Float;
    fn sub(self, rhs: &'rhs Float) -> Self::Output {
        Float::from(f64::from(self) - f64::from(rhs))
    }
}

impl<'lhs, 'rhs> ops::Sub<&'rhs f64> for &'lhs Float {
    type Output = Float;
    fn sub(self, rhs: &'rhs f64) -> Self::Output {
        Float::from(f64::from(self) - rhs)
    }
}

impl<'lhs, 'rhs> ops::Sub<&'rhs Float> for &'lhs f64 {
    type Output = f64;
    fn sub(self, rhs: &'rhs Float) -> Self::Output {
        *self - f64::from(rhs)
    }
}
