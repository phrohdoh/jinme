use crate::prelude::*;
use ::std::cmp;

// Value Comparisions for Floats & f64s (lhs owned, rhs owned)
// -----------------------------------------------------------------------------

impl cmp::PartialEq<Float> for Float {
    fn eq(&self, rhs: &Float) -> bool {
        f64::from(self) == f64::from(rhs)
    }
}

impl cmp::PartialEq<f64> for Float {
    fn eq(&self, rhs: &f64) -> bool {
        f64::from(self) == *rhs
    }
}

impl cmp::PartialEq<Float> for f64 {
    fn eq(&self, rhs: &Float) -> bool {
        *self == f64::from(rhs)
    }
}
