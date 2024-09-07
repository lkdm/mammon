#![cfg_attr(not(feature = "std"), no_std)]
use core::ops::{Add, Div, Mul, Rem, Sub};
use num_traits::{FromPrimitive, PrimInt, ToPrimitive};

trait MoneyOps<T> {
    fn parse_string(s: &str) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Money<T: PrimInt + FromPrimitive + ToPrimitive>(T);

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

impl<T: PrimInt + FromPrimitive + ToPrimitive> Money<T> {
    pub fn new(value: T) -> Self {
        Money(value)
    }
    fn div_round_up(a: T, b: T) -> T {
        (a + b - T::one()) / b
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Add for Money<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Money(self.0 + rhs.0)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Sub for Money<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Money(self.0 - rhs.0)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Mul for Money<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let hundred = T::from_u8(100).unwrap();
        let result = Self::div_round_up(self.0 * rhs.0, hundred);
        Money(result)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Div for Money<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let hundred = T::from_u8(100).unwrap();
        let result = Self::div_round_up(self.0 * hundred, rhs.0);
        Money(result)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Rem for Money<T> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Money(self.0 % rhs.0)
    }
}

pub type Moneyi128 = Money<i128>;
pub type Moneyisize = Money<isize>;
pub type Moneyi64 = Money<i64>;
pub type Moneyi32 = Money<i32>;
pub type Moneyi16 = Money<i16>;
pub type Moneyi8 = Money<i8>;
pub type Moneyusize = Money<usize>;
pub type Moneyu128 = Money<u128>;
pub type Moneyu64 = Money<u64>;
pub type Moneyu32 = Money<u32>;
pub type Moneyu16 = Money<u16>;
pub type Moneyu8 = Money<u8>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_money() {
        let m1 = Moneyi32::new(123);
        let m2 = Moneyi32::new(456);
        assert_eq!(m1 + m2, Money(579));
        assert_eq!(m1 - m2, Money(-333));
        assert_eq!(m1 * m2, Money(561));
        assert_eq!(m1 / m2, Money(27));
        assert_eq!(m1 % m2, Money(123));
    }
}

// impl Money {
//     /// Parse a string into Money, handling conversion to cents.
//     fn parse_string(s: &str) -> u32 {
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

//         let integer_value: u32 = integer_part.parse().unwrap_or(0);
//         let fractional_value: u32 = fractional_part.parse().unwrap_or(0);

//         integer_value * 100 + fractional_value
//     }
// }
