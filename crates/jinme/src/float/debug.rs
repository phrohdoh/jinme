use crate::prelude::*;
use ::std::fmt;

impl fmt::Debug for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let (mantissa, exponent, sign) = &self.0;
        // write!(f, "({:?}, {:?}, {:?})", mantissa, exponent, sign)
        write!(f, "{:?}", f64::from(self))
    }
}
