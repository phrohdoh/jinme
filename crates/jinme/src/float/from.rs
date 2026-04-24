use crate::float::*;

// f64
// -----------------------------------------------------------------------------

impl From<f64> for Float {
    fn from(src: f64) -> Self {
        Self(decode(src))
    }
}

impl From<Float> for f64 {
    fn from(src: Float) -> Self {
        let (mantissa, exponent, sign) = src.0;
        encode(mantissa, exponent, sign)
    }
}

impl From<&f64> for Float {
    fn from(src: &f64) -> Self {
        Self(decode(*src))
    }
}

impl From<&Float> for f64 {
    fn from(src: &Float) -> Self {
        let (mantissa, exponent, sign) = src.0.to_owned();
        encode(mantissa, exponent, sign)
    }
}

// Mantissa
// -----------------------------------------------------------------------------

impl From<u64> for Mantissa {
    fn from(mantissa: u64) -> Self {
        Self(mantissa)
    }
}

impl From<Mantissa> for u64 {
    fn from(mantissa: Mantissa) -> Self {
        mantissa.0
    }
}

// Exponent
// -----------------------------------------------------------------------------

impl From<i16> for Exponent {
    fn from(exponent: i16) -> Self {
        Self(exponent)
    }
}

impl From<Exponent> for i16 {
    fn from(exponent: Exponent) -> Self {
        exponent.0
    }
}

// Sign
// -----------------------------------------------------------------------------

impl From<i8> for Sign {
    fn from(exponent: i8) -> Self {
        Self(exponent)
    }
}

impl From<Sign> for i8 {
    fn from(exponent: Sign) -> Self {
        exponent.0
    }
}
