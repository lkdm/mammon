#![cfg_attr(not(feature = "std"), no_std)]
use core::ops::{Add, Div, Mul, Rem, Sub, Deref};
use num_traits::{FromPrimitive, PrimInt, ToPrimitive};

trait MoneyOps<T> {
    fn parse_string(s: &str) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cent<T: PrimInt + FromPrimitive + ToPrimitive>(T);

impl<T: PrimInt + FromPrimitive + ToPrimitive> Deref for Cent<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PrimInt + FromPrimitive> From<f32> for Cent<T> {
    fn from(f: f32) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 100.0).round();
        let cents = T::from_f32(rounded).expect("Failed to convert f32 to T");
        Cent::new(cents)
    }
}
impl<T: PrimInt + FromPrimitive> From<f64> for Cent<T> {
    fn from(f: f64) -> Self {
        // Multiply by 100 and round to the nearest cent
        let rounded = (f * 100.0).round();
        let cents = T::from_f64(rounded).expect("Failed to convert f64 to T");
        Cent::new(cents)
    }
}

// impl<T: PrimInt + FromPrimitive + ToPrimitive> MoneyOps<Money<T>> for Money<T> {
//     fn parse_string(s: &str) -> T {
//         let mut integer_part = String::new();
//         let mut fractional_part = String::new();
//         let mut found_decimal = false;

//         for c in s.chars() {
//             if c.is_digit(10) {
//                 if found_decimal {
//                     fractional_part.push(c);
//                 } else {
//                     integer_part.push(c);
//                 }
//             } else if c == '.' {
//                 if found_decimal {
//                     // Ignore additional decimal points
//                     continue;
//                 }
//                 found_decimal = true;
//             }
//         }

//         // Ensure fractional part is exactly two digits
//         if fractional_part.len() == 1 {
//             fractional_part.push('0'); // Pad with zero if only one digit
//         } else if fractional_part.len() > 2 {
//             fractional_part.truncate(2); // Truncate to two digits
//         }

//         let integer_value: T = integer_part.parse().unwrap_or(0);
//         let fractional_value: T = fractional_part.parse().unwrap_or(0);

//         integer_value * 100 + fractional_value
//     }
// }

impl<T: PrimInt + FromPrimitive + ToPrimitive> Cent<T> {
    pub fn new(value: T) -> Self {
        Cent(value)
    }
    fn div_round_up(a: T, b: T) -> T {
        (a + b - T::one()) / b
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Add for Cent<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Cent(self.0 + rhs.0)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Sub for Cent<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Cent(self.0 - rhs.0)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Mul for Cent<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let hundred = T::from_u8(100).unwrap();
        let result = Self::div_round_up(self.0 * rhs.0, hundred);
        Cent(result)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Div for Cent<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let hundred = T::from_u8(100).unwrap();
        let result = Self::div_round_up(self.0 * hundred, rhs.0);
        Cent(result)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Rem for Cent<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Cent(self.0 % rhs.0)
    }
}

pub type Centi128 = Cent<i128>;
pub type Centisize = Cent<isize>;
pub type Centi64 = Cent<i64>;
pub type Centi32 = Cent<i32>;
pub type Centi16 = Cent<i16>;
pub type Centi8 = Cent<i8>;
pub type Centusize = Cent<usize>;
pub type Centu128 = Cent<u128>;
pub type Centu64 = Cent<u64>;
pub type Centu32 = Cent<u32>;
pub type Centu16 = Cent<u16>;
pub type Centu8 = Cent<u8>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_money() {
        let m1 = Centi32::new(123);
        let m2 = Centi32::new(456);
        assert_eq!(m1 + m2, Cent(579));
        assert_eq!(m1 - m2, Cent(-333));
        assert_eq!(m1 * m2, Cent(561));
        assert_eq!(m1 / m2, Cent(27));
        assert_eq!(m1 % m2, Cent(123));
    }

    #[test]
    fn test_int_interop() {
        let m1 = Centi32::new(123);
        let n1: f32 = 4.56;
        assert_eq!(m1 + n1.into(), Cent(579));
        assert_eq!(m1 - n1.into(), Cent(-333));
        assert_eq!(m1 * n1.into(), Cent(561));
        assert_eq!(m1 / n1.into(), Cent(27));
        assert_eq!(m1 % n1.into(), Cent(123));
    }
}
