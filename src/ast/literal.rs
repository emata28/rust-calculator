use super::base_node::BaseNode;
use super::numeric::Numeric;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Literal {
    value: Box<dyn Numeric>,
}

impl Literal {
    pub fn new(value: impl Numeric + Copy + 'static) -> Self {
        Self {
            value: Box::new(value),
        }
    }
}

impl BaseNode for Literal {
    fn evaluate(&self) -> Box<dyn Numeric> {
        self.value.clone_box()
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.evaluate())
    }
}
