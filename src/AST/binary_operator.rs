use super::base_node::{BaseNode, Numeric};

pub struct BinaryOperator<T: Numeric> {
    left: Box<dyn BaseNode<T>>,
    right: Box<dyn BaseNode<T>>,
    operator: String,
}

impl<T: Numeric> BinaryOperator<T> {
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

impl<T: Numeric> BaseNode<T> for BinaryOperator<T> {
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
