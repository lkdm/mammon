#![cfg_attr(not(feature = "std"), no_std)]
use core::fmt;
use core::ops::{Add, Deref, Div, Mul, Rem, Sub};
use num_traits::{FromPrimitive, PrimInt, Signed, ToPrimitive};

/// Mills
///
/// Mills is a newtype wrapper around a primitive integer type that represents a value.
/// The value is stored in 1/1000th of the base unit– ie. 1/1000th of a dollar.
///
/// ## Usage
/// You can use Mills to declare your own wrapper for a primitive integer type, or use the provided ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mills<T: PrimInt + FromPrimitive + ToPrimitive>(T);

impl<T: PrimInt + FromPrimitive + ToPrimitive> Deref for Mills<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PrimInt + FromPrimitive> From<f32> for Mills<T> {
    /// Convert a f32 to Mills
    fn from(f: f32) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 1_000.0).round();
        let cents = T::from_f32(rounded).expect("Failed to convert f32 to T");
        Mills::new(cents)
    }
}
impl<T: PrimInt + FromPrimitive> From<f64> for Mills<T> {
    /// Convert a f64 to Mills
    fn from(f: f64) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 1_000.0).round();
        let cents = T::from_f64(rounded).expect("Failed to convert f64 to T");
        Mills::new(cents)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Mills<T> {
    /// Create a new Mill
    ///
    /// Be careful to supply the correct value in Mills, as it is not automatically converted.
    pub fn new(value: T) -> Self {
        Mills(value)
    }

    /// # From Cents
    /// Creates a new Mill from a value in cents
    ///
    /// Some APIs send values in cents, this function can be used to convert them to Mills.
    pub fn from_cents(cents: T) -> Self {
        Mills(cents * T::from_u16(10).unwrap())
    }

    fn round_bankers(value: T) -> T {
        let precision = T::from_u16(1_000).unwrap();
        let half = T::from_u16(500).unwrap();
        let remainder = value % precision;

        if remainder > half
            || (remainder == half && (value / precision) % T::from_u8(2).unwrap() == T::one())
        {
            value + precision - remainder
        } else {
            value - remainder
        }
    }
}

impl<T> fmt::Display for Mills<T>
where
    T: PrimInt + FromPrimitive + Div<Output = T> + Rem<Output = T> + Signed,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let thousand = T::from_u32(1_000).unwrap();
        let abs_value = self.0.abs();
        let dollars = abs_value / thousand;
        let mills = abs_value % thousand;

        let sign = if self.0.is_negative() { "-" } else { "" };

        write!(
            f,
            "{}${}.{:03}",
            sign,
            dollars.to_u64().unwrap_or(0),
            mills.to_u64().unwrap_or(0)
        )
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Add for Mills<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Mills(self.0 + rhs.0)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Sub for Mills<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Mills(self.0 - rhs.0)
    }
}
impl<T: PrimInt + FromPrimitive + ToPrimitive> Mul for Mills<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let precision = T::from_u16(1_000).unwrap();
        // Perform multiplication with increased precision
        let result = self.0 * rhs.0;
        // Round the result
        let rounded = Self::round_bankers(result);
        // Divide by precision to get back to Mills
        Mills(rounded / precision)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Div for Mills<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let precision = T::from_u16(1_000).unwrap();
        // Perform division with increased precision
        let result = self.0 * precision * precision / rhs.0;
        // Round the result
        let rounded = Self::round_bankers(result);
        // Divide by precision to get back to Mills
        Mills(rounded / precision)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Rem for Mills<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Mills(self.0 % rhs.0)
    }
}

/// Milli64
/// A Milli wrapper around i64
pub type Milli64 = Mills<i64>;
/// Milliu64
/// A Milli wrapper around u64
pub type Millu64 = Mills<u64>;
/// Milli128
/// A Milli wrapper around i128
pub type Milli128 = Mills<i128>;
/// Milliu128
/// A Milli wrapper around u128
pub type Millu128 = Mills<u128>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_money() {
        let m1 = Milli64::new(1_234);
        let m2 = Milli64::new(4_567);
        assert_eq!(m1 + m2, Milli64::new(5_801));
        assert_eq!(m1 - m2, Milli64::new(-3_333));
        assert_eq!(m1 * m2, Milli64::new(5_636));
        assert_eq!(m1 / m2, Milli64::new(270));
        assert_eq!(m1 % m2, Milli64::new(1_234));
    }

    #[test]
    fn test_int_interop() {
        let m1 = Milli64::new(0_123);
        let n1: f32 = 4.56;
        assert_eq!(m1 + n1.into(), Milli64::new(4_683));
        assert_eq!(m1 - n1.into(), Milli64::new(-4_437));
        assert_eq!(m1 * n1.into(), Milli64::new(561));
        assert_eq!(m1 / n1.into(), Milli64::new(27));
        assert_eq!(m1 % n1.into(), Milli64::new(123));
    }

    #[test]
    fn test_from_cents() {
        let m1 = Milli64::from_cents(123);
        assert_eq!(m1, Milli64::new(1_230));
    }

    #[test]
    fn test_display() {
        let m1 = Milli64::new(1_234);
        assert_eq!(format!("{}", m1), "$1.234");
        let m2 = Milli64::new(-1_234);
        assert_eq!(format!("{}", m2), "-$1.234");
    }
}
