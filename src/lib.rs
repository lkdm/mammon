#![cfg_attr(not(feature = "std"), no_std)]
use core::ops::{Add, Div, Mul, Rem, Sub, Deref};
use num_traits::{FromPrimitive, PrimInt, ToPrimitive};

/// Mill
///
/// A Mill is a fixed-point number with 64 bits of precision and a scale of 1,000.
/// (2^64/2-1)/1000 = 9,223,372,036,854,775.807
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Mills<T: PrimInt + FromPrimitive + ToPrimitive>(T);

impl<T: PrimInt + FromPrimitive + ToPrimitive> Deref for Mills<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PrimInt + FromPrimitive> From<f32> for Mills<T> {
    fn from(f: f32) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 1_000.0).round();
        let cents = T::from_f32(rounded).expect("Failed to convert f32 to T");
        Mills::new(cents)
    }
}
impl<T: PrimInt + FromPrimitive> From<f64> for Mills<T> {
    fn from(f: f64) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 1_000.0).round();
        let cents = T::from_f64(rounded).expect("Failed to convert f64 to T");
        Mills::new(cents)
    }
}

// // impl<T: PrimInt + FromPrimitive + ToPrimitive> MoneyOps<Money<T>> for Money<T> {
// //     fn parse_string(s: &str) -> T {
// //         let mut integer_part = String::new();
// //         let mut fractional_part = String::new();
// //         let mut found_decimal = false;

// //         for c in s.chars() {
// //             if c.is_digit(10) {
// //                 if found_decimal {
// //                     fractional_part.push(c);
// //                 } else {
// //                     integer_part.push(c);
// //                 }
// //             } else if c == '.' {
// //                 if found_decimal {
// //                     // Ignore additional decimal points
// //                     continue;
// //                 }
// //                 found_decimal = true;
// //             }
// //         }

// //         // Ensure fractional part is exactly two digits
// //         if fractional_part.len() == 1 {
// //             fractional_part.push('0'); // Pad with zero if only one digit
// //         } else if fractional_part.len() > 2 {
// //             fractional_part.truncate(2); // Truncate to two digits
// //         }

// //         let integer_value: T = integer_part.parse().unwrap_or(0);
// //         let fractional_value: T = fractional_part.parse().unwrap_or(0);

// //         integer_value * 100 + fractional_value
// //     }
// // }

impl<T: PrimInt + FromPrimitive + ToPrimitive> Mills<T> {
    pub fn new(value: T) -> Self {
        Mills(value)
    }
    fn round_bankers(value: T) -> T {
        let precision = T::from_u16(1_000).unwrap();
        let half = T::from_u16(500).unwrap();
        let remainder = value % precision;

        if remainder > half || (remainder == half && (value / precision) % T::from_u8(2).unwrap() == T::one()) {
            value + precision - remainder
        } else {
            value - remainder
        }
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

// impl<T: PrimInt + FromPrimitive + ToPrimitive> Mul for Mills<T> {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self::Output {
//         let precision = T::from_u16(1_000).unwrap();
//         let result = self.0 * rhs.0 / precision;
//         Mills(result)
//     }
// }

// impl<T: PrimInt + FromPrimitive + ToPrimitive> Div for Mills<T> {
//     type Output = Self;
//     fn div(self, rhs: Self) -> Self::Output {
//         let precision = T::from_u16(1_000).unwrap();
//         let result = self.0 * precision / rhs.0;
//         Mills(result)
//     }
// }

impl<T: PrimInt + FromPrimitive + ToPrimitive> Rem for Mills<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Mills(self.0 % rhs.0)
    }
}

pub type Milli64 = Mills<i64>;
pub type Millu64 = Mills<u64>;
pub type Milli128 = Mills<i128>;
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
}
