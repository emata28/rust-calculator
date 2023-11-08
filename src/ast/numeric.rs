use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Numeric {
    Float(f64),
    Integer(i64),
}

impl Clone for Numeric {
    fn clone(&self) -> Self {
        match self {
            Numeric::Float(value) => Numeric::Float(*value),
            Numeric::Integer(value) => Numeric::Integer(*value),
        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Numeric::Float(value) => write!(f, "{}", value),
            Numeric::Integer(value) => write!(f, "{}", value),
        }
    }
}

impl Add for Numeric {
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        let left = self;
        let right = rhs;

        match (left, right) {
            (Numeric::Float(left), Numeric::Float(right)) => Numeric::Float(left + right),
            (Numeric::Integer(left), Numeric::Integer(right)) => Numeric::Integer(left + right),
            (Numeric::Float(left), Numeric::Integer(right)) => Numeric::Float(left + right as f64),
            (Numeric::Integer(left), Numeric::Float(right)) => Numeric::Float(left as f64 + right),
        }
    }
}

impl Sub for Numeric {
    type Output = Numeric;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self;
        let right = rhs;

        match (left, right) {
            (Numeric::Float(left), Numeric::Float(right)) => Numeric::Float(left - right),
            (Numeric::Integer(left), Numeric::Integer(right)) => Numeric::Integer(left - right),
            (Numeric::Float(left), Numeric::Integer(right)) => Numeric::Float(left - right as f64),
            (Numeric::Integer(left), Numeric::Float(right)) => Numeric::Float(left as f64 - right),
        }
    }
}

impl Div for Numeric {
    type Output = Numeric;

    fn div(self, rhs: Self) -> Self::Output {
        let left = self;
        let right = rhs;

        match (left, right) {
            (Numeric::Float(left), Numeric::Float(right)) => Numeric::Float(left / right),
            (Numeric::Integer(left), Numeric::Integer(right)) => Numeric::Integer(left / right),
            (Numeric::Float(left), Numeric::Integer(right)) => Numeric::Float(left / right as f64),
            (Numeric::Integer(left), Numeric::Float(right)) => Numeric::Float(left as f64 / right),
        }
    }
}

impl Mul for Numeric {
    type Output = Numeric;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = self;
        let right = rhs;

        match (left, right) {
            (Numeric::Float(left), Numeric::Float(right)) => Numeric::Float(left * right),
            (Numeric::Integer(left), Numeric::Integer(right)) => Numeric::Integer(left * right),
            (Numeric::Float(left), Numeric::Integer(right)) => Numeric::Float(left * right as f64),
            (Numeric::Integer(left), Numeric::Float(right)) => Numeric::Float(left as f64 * right),
        }
    }
}

impl FromStr for Numeric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            s.parse::<f64>()
                .map(Numeric::Float)
                .map_err(|_| format!("Failed to parse token: {}", s))
        } else {
            s.parse::<i64>()
                .map(Numeric::Integer)
                .map_err(|_| format!("Failed to parse token: {}", s))
        }
    }
}
