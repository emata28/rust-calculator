use super::base_node::BaseNode;
use std::ops::{Add, Div, Mul, Sub};
pub struct BinaryOperator<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>>
{
    left: Box<dyn BaseNode<T>>,
    right: Box<dyn BaseNode<T>>,
    operator: String,
}

impl<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>> BinaryOperator<T> {
    pub fn new(left: Box<dyn BaseNode<T>>, right: Box<dyn BaseNode<T>>, operator: String) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
    pub fn eval(&self) -> T {
        self.evaluate()
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Div<Output = T> + Mul<Output = T>> BaseNode<T>
    for BinaryOperator<T>
{
    fn evaluate(&self) -> T {
        match self.operator.as_str() {
            "+" => self.left.evaluate() + self.right.evaluate(),
            "-" => self.left.evaluate() - self.right.evaluate(),
            "*" => self.left.evaluate() * self.right.evaluate(),
            "/" => self.left.evaluate() / self.right.evaluate(),
            _ => panic!("Unknown operator"),
        }
    }
}
