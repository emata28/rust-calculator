use super::base_node::{BaseNode, Numeric};

pub struct Literal<T: Numeric> {
    value: T,
}

impl<T: Numeric> Literal<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Numeric> BaseNode<T> for Literal<T> {
    fn evaluate(&self) -> T {
        self.value
    }
}
