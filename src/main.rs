mod ast;

use ast::{binary_operator::BinaryOperator, literal::Literal, Ast};

pub mod calc;

fn main() {
    let a = Literal::new(1);
    let b = Literal::new(2);
    let _c = Literal::new(3);

    let bin_op = BinaryOperator::new(Box::new(a), Box::new(b), "+".to_string());

    println!("{}", bin_op);

    let root = Ast::new("1 + 2 / 2".to_string());
}
