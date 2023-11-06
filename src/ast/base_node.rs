use std::fmt::{Display, Formatter, Result as FmtResult};

use super::numeric::Numeric;

pub trait BaseNode {
    fn evaluate(&self) -> Numeric;
}

impl Display for dyn BaseNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.evaluate())
    }
}
