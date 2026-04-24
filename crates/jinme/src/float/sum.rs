use crate::prelude::*;
use ::std::iter;

impl iter::Sum<Float> for Float {
    fn sum<I: Iterator<Item = Float>>(iter: I) -> Self {
        iter.fold(Float::from(0.0), |acc, x| acc + x)
    }
}

impl<'a> iter::Sum<&'a Float> for Float {
    fn sum<I: Iterator<Item = &'a Float>>(iter: I) -> Self {
        iter.fold(Float::from(0.0), |acc, x| acc + x)
    }
}
