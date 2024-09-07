use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money(u32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rate(u32, u8);

impl Money {
    /// Parse a string into Money, handling conversion to cents.
    fn parse_string(s: &str) -> u32 {
        let mut integer_part = String::new();
        let mut fractional_part = String::new();
        let mut found_decimal = false;

        for c in s.chars() {
            if c.is_digit(10) {
                if found_decimal {
                    fractional_part.push(c);
                } else {
                    integer_part.push(c);
                }
            } else if c == '.' {
                if found_decimal {
                    // Ignore additional decimal points
                    continue;
                }
                found_decimal = true;
            }
        }

        // Ensure fractional part is exactly two digits
        if fractional_part.len() == 1 {
            fractional_part.push('0'); // Pad with zero if only one digit
        } else if fractional_part.len() > 2 {
            fractional_part.truncate(2); // Truncate to two digits
        }

        let integer_value: u32 = integer_part.parse().unwrap_or(0);
        let fractional_value: u32 = fractional_part.parse().unwrap_or(0);

        integer_value * 100 + fractional_value
    }
}

impl From<String> for Money {
    fn from(s: String) -> Self {
        // Clean up the string and handle invalid cases
        let parsed_value = Money::parse_string(&s);
        Money(parsed_value)
    }
}

impl Add<Money> for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Money {
        Money(self.0 + rhs.0)
    }
}

impl Sub<Money> for Money {
    type Output = Money;

    fn sub(self, rhs: Money) -> Money {
        Money(self.0 - rhs.0)
    }
}

impl Mul<Money> for Money {
    type Output = Money;

    fn mul(self, rhs: Money) -> Money {
        Money((self.0 * rhs.0) / 100)
    }
}

impl Div<Money> for Money {
    type Output = Money;

    fn div(self, rhs: Money) -> Money {
        Money(self.0 * 100 / rhs.0)
    }
}

impl AddAssign<Money> for Money {
    fn add_assign(&mut self, other: Money) {
        self.0 += other.0;
    }
}

impl SubAssign<Money> for Money {
    fn sub_assign(&mut self, other: Money) {
        self.0 -= other.0;
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dollars = self.0 / 100;
        let cents = self.0 % 100;
        write!(f, "${:}.{:02}", dollars, cents)
    }
}
