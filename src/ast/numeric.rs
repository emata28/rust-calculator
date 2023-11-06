use num_traits::Num;
use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{Display, Formatter, Result as FmtResult};


pub trait Numeric {
    fn as_string(&self) -> String;
    fn clone_box(&self) -> Box<dyn Numeric>;
}

impl<T> Numeric for T
    where
        T: Num + Clone + 'static + Display,
{
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn clone_box(&self) -> Box<dyn Numeric> {
        Box::new(self.clone())
    }
}

impl Add for Box<dyn Numeric> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let left = self.clone_box();
        let right = rhs.clone_box();

        let left = left.as_string().parse::<f64>().unwrap();
        let right = right.as_string().parse::<f64>().unwrap();

        Box::new(left + right)
    }
}

impl Sub for Box<dyn Numeric> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let left = self.clone_box();
        let right = rhs.clone_box();

        let left = left.as_string().parse::<f64>().unwrap();
        let right = right.as_string().parse::<f64>().unwrap();

        Box::new(left - right)
    }
}

impl Mul for Box<dyn Numeric> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let left = self.clone_box();
        let right = rhs.clone_box();

        let left = left.as_string().parse::<f64>().unwrap();
        let right = right.as_string().parse::<f64>().unwrap();

        Box::new(left * right)
    }
}

impl Div for Box<dyn Numeric> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let left = self.clone_box();
        let right = rhs.clone_box();

        let left = left.as_string().parse::<f64>().unwrap();
        let right = right.as_string().parse::<f64>().unwrap();

        Box::new(left / right)
    }
}

impl Display for dyn Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.as_string())
    }
}