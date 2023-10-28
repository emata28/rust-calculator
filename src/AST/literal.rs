use super::base_node::BaseNode;

use std::ops::{Add, Div, Mul, Sub};

pub struct Literal<T: Add + Div + Mul + Sub> {
    value: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>> Literal<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>> BaseNode<T>
    for Literal<T>
{
    fn evaluate(&self) -> T {
        self.value
    }
}
