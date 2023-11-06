use super::base_node::BaseNode;
use super::numeric::Numeric;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Literal {
    value: Numeric,
}

impl Literal {
    pub fn new(value: Numeric) -> Self {
        Self { value }
    }
}

impl BaseNode for Literal {
    fn evaluate(&self) -> Numeric {
        self.value.clone()
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.evaluate())
    }
}
