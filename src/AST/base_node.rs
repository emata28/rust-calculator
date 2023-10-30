use std::ops::{Add, Div, Mul, Sub};

pub trait Numeric:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Copy
{
}

impl<T> Numeric for T where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Copy
{
}

pub trait BaseNode<T: Numeric> {
    fn evaluate(&self) -> T;
}
