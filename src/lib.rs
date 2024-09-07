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
        Money((self.0 * rhs.0) >> 6)
    }
}

impl<T: PrimInt + FromPrimitive + ToPrimitive> Div for Money<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let numerator = (self.0 << 6) + (self.0 << 5) + (self.0 << 2);
        Money(numerator / rhs.0)
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

mod tests {
    use super::*;
    #[cfg(test)]
    fn test_money() {
        let m1 = Moneyi32::new(123);
        let m2 = Moneyi32::new(456);
        assert_eq!(m1 + m2, Money(679));
        assert_eq!(m1 - m2, Money(-333));
        assert_eq!(m1 * m2, Money(56));
        assert_eq!(m1 / m2, Money(26));
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
