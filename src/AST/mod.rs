pub mod base_node;
pub mod binary_operator;
pub mod literal;

pub use base_node::{BaseNode, Numeric};
pub use binary_operator::BinaryOperator;
pub use literal::Literal;

pub struct Ast {
    pub root: Box<dyn BaseNode>,
}

impl Ast {
    pub fn new(expression: String) -> Self {
        let tokens = Self::tokenize(expression);

        let root: Box<dyn BaseNode> = Box::new(Literal::new(1));
        Self { root }
    }

    fn tokenize(expression: String) -> Vec<String> {
        let tokens: Vec<String> = expression
            .replace(' ', "")
            .split("")
            .map(|s| s.to_string())
            .collect();
        tokens
    }

    pub fn add(node: Box<dyn BaseNode>) {}
}
