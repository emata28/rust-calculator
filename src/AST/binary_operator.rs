use super::base_node::{BaseNode, Numeric};
use std::fmt::{Display, Formatter, Result};

pub struct BinaryOperator {
    left: Box<dyn BaseNode>,
    right: Box<dyn BaseNode>,
    operator: String,
}

impl BinaryOperator {
    pub fn new(left: Box<dyn BaseNode>, right: Box<dyn BaseNode>, operator: String) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }
}

impl BaseNode for BinaryOperator {
    fn evaluate(&self) -> Box<dyn Numeric> {
        let left = self.left.evaluate();
        let right = self.right.evaluate();
        let operator = &self.operator;

        match operator.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => panic!("Unknown operator {}", operator),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} {} {} = {}",
            self.left,
            self.operator,
            self.right,
            self.evaluate()
        )
    }
}
