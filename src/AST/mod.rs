pub mod base_node;
pub mod binary_operator;
pub mod literal;

pub use base_node::{BaseNode, Numeric};
pub use binary_operator::BinaryOperator;
pub use literal::Literal;

pub struct Ast<T> {
    pub root: Box<dyn BaseNode<T>>,
}

impl<T: Numeric> Ast<T> {
    pub fn new(root: Box<dyn BaseNode<T>>) -> Self {
        Self { root }
    }
}
