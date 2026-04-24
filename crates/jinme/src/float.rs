mod add;
mod debug;
mod display;
mod div;
mod eq;
mod from;
mod mul;
mod product;
mod sub;
mod sum;

type Components = (Mantissa, Exponent, Sign);

// TODO: manually impl Ord, Eq, etc (defer to f64 impls after encoding)
// #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[derive(Hash, Ord, PartialOrd, Eq, Clone)]
pub struct Float(Components);

/// Custom floating-point number representation using mantissa, exponent, and sign.
///
/// This provides a custom representation of floating-point numbers with explicit
/// control over mantissa, exponent, and sign components. This can be useful for
/// debugging, serialization, or implementing custom floating-point operations.
///
/// # Components
///
/// - `Mantissa`: The fractional part (53 bits for IEEE 754 double precision)
/// - `Exponent`: The unbiased exponent
/// - `Sign`: The sign of the number (1 for positive, -1 for negative)
///
/// # Example
///
/// ```
/// # use jinme::prelude::*;
/// let float = Float::try_from("3.14").unwrap();
/// assert_eq!(float.as_f64(), 3.14);
/// ```
impl Float {
    pub fn mantissa(&self) -> Mantissa {
        self.0.0.clone()
    }
    pub fn exponent(&self) -> Exponent {
        self.0.1.clone()
    }
    pub fn sign(&self) -> Sign {
        self.0.2.clone()
    }
    pub fn as_f64(&self) -> f64 {
        encode(self.mantissa(), self.exponent(), self.sign())
    }
}

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct Mantissa(u64);

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct Exponent(i16);

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq, Debug, Clone)]
pub struct Sign(i8);

impl TryFrom<&str> for Float {
    type Error = std::num::ParseFloatError;
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let native_float64: f64 = string.parse()?;
        Ok(Self(decode(native_float64)))
    }
}

/// Decode a `f64` into the `mantissa`, `exponent`, and `sign` components.
pub fn decode(native_float64: f64) -> (Mantissa, Exponent, Sign) {
    //let bits: u64 = unsafe { mem::transmute(native_float64) };
    let bits = native_float64.to_bits();
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };
    exponent -= 1023 + 52;
    (mantissa.into(), exponent.into(), sign.into())
}

/// Re‑assemble a `f64` from the components produced by `decode`.
///
/// * `mantissa`: the 53‑bit mantissa (including the implicit leading 1 for
///   normal numbers).  
/// * `exponent`: the unbiased exponent (the value after the `‑1023‑52` bias
///   applied in `decode`).  
/// * `sign`: `0` for `0`, `1` for positive numbers, `‑1` for negative numbers.
///
/// Returns the reconstructed `f64`.  The function mirrors the logic of
/// `decode`, handling both normal and subnormal values.
pub fn encode(mantissa: Mantissa, exponent: Exponent, sign: Sign) -> f64 {
    let mantissa: u64 = mantissa.into();
    let exponent: i16 = exponent.into();
    let sign: i8 = sign.into();

    // (1) Determine the sign bit.
    let sign_bit: u64 = if sign < 0 { 1 } else { 0 } << 63;

    // (2) Re‑bias the exponent.
    // `decode` performed:  exponent -= 1023 + 52;
    // To reverse that we add the bias back.
    let biased_exp = (exponent + 1023 + 52) as u64;

    // (3) Split the mantissa back into the stored fraction and the hidden bit.
    // The original function left‑shifted the fraction for subnormals
    // (exponent == 0) and OR‑ed the hidden bit for normals.
    // We need to detect which case we are dealing with.
    let (exp_field, frac_field) = if biased_exp == 0 {
        // Subnormal: the hidden bit is not present, the mantissa was left‑shifted
        // by one, so we shift it back.
        (0u64, mantissa >> 1)
    } else {
        // Normal: the hidden bit (bit 52) is present in `mantissa`.
        // Clear that bit to obtain the stored fraction.
        let frac = mantissa & 0x0F_FF_FF_FF_FF_FF_FF; // lower 52 bits
        (biased_exp, frac)
    };

    // (4) Assemble the IEEE‑754 bit pattern.
    let bits = sign_bit | (exp_field << 52) | frac_field;

    // (5) Convert the raw bits back to `f64`.
    //let native_float64 = unsafe { mem::transmute(bits) };
    let native_float64 = f64::from_bits(bits);
    native_float64
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn add() {
        // arrange
        let input = Float::from(5.0);
        let add = Float::from(2.0);
        // act
        let output = input + add;
        // assert
        assert_eq!(output, Float::from(7.0));
    }

    #[test]
    fn sub() {
        // arrange
        let input = Float::from(5.0);
        let sub = Float::from(2.0);
        // act
        let output = input - sub;
        // assert
        assert_eq!(output, Float::from(3.0));
    }

    #[test]
    fn mul() {
        // arrange
        let input = Float::from(5.0);
        let mul = Float::from(2.0);
        // act
        let output = input * mul;
        // assert
        assert_eq!(output, Float::from(10.0));
    }

    #[test]
    fn div() {
        // arrange
        let input = Float::from(5.0);
        let div = Float::from(2.0);
        // act
        let output = input / div;
        // assert
        assert_eq!(output, Float::from(2.5));
    }
}
