use std::{fmt, ops::Add, iter::{repeat, Sum}};

pub mod input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Digit(i8);
pub struct SnafuNumber(Vec<Digit>);

impl From<u8> for Digit {
    #[inline]
    fn from(byte: u8) -> Self {
        if byte >= b'0' && byte <= b'2' {
            Digit(byte as i8 - 48)
        } else if byte == b'-' {
            Digit(-1)
        } else if byte == b'=' {
            Digit(-2)
        } else {
            unreachable!("Wrong digit!")
        }
    }
}

impl Add for Digit {
    type Output = (Digit, Digit); // (Carry, AddResult)

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let result = self.0 + rhs.0;
        if result < -2 {
            (Digit(-1), Digit(result + 5))
        } else if result > 2 {
            (Digit(1), Digit(result - 5))
        } else {
            (Digit(0), Digit(result))
        }
    }
}

impl fmt::Display for Digit {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 0 && self.0 <= 2 {
            write!(f, "{}", self.0)?;
        } else if self.0 == -1 {
            write!(f, "-")?;
        } else if self.0 == -2 {
            write!(f, "=")?;
        } else {
            unreachable!()
        }
        Ok(())
    }
}

impl From<&str> for SnafuNumber {
    #[inline]
    fn from(line: &str) -> Self {
        SnafuNumber(line.bytes().map(|line| line.into()).collect())
    }
}

impl Add for SnafuNumber {
    type Output = SnafuNumber;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let digits = self.0.len().max(rhs.0.len());
        let lhs_iter = self.0.iter().rev().copied().chain(repeat(Digit(0)).take(digits - self.0.len()));
        let rhs_iter = rhs.0.iter().rev().copied().chain(repeat(Digit(0)).take(digits - rhs.0.len()));

        let (carry, mut digits) = lhs_iter.zip(rhs_iter).fold((Digit(0), Vec::new()), |(carry, mut digits), (digit_left, digit_right)| {
            let (next_carry1, next_digit) = carry + digit_left;
            let (next_carry2, next_digit) = next_digit + digit_right;
            digits.insert(0, next_digit);

            (Digit(next_carry1.0 + next_carry2.0), digits)
        });

        if carry != Digit(0) {
            digits.insert(0, carry);
        }
        SnafuNumber(digits)
    }
}

impl Sum for SnafuNumber {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|prev, next| prev + next).unwrap()
    }
}

impl fmt::Display for SnafuNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for digit in self.0.iter() {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

#[inline]
pub fn sum(input: &str) -> String {
    input.lines().map(|line| SnafuNumber::from(line)).sum::<SnafuNumber>().to_string()
}