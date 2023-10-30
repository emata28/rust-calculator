mod ast;

use ast::{binary_operator::BinaryOperator, literal::Literal, Ast};
pub mod calc;

fn main() {
    let a = Literal::new(1f32);
    let b = Literal::new(2f32);
    let _c = Literal::new(3f32);

    let bin_op = BinaryOperator::new(Box::new(a), Box::new(b), "+".to_string());

    println!("{}", bin_op.eval());

    let root = Ast::new(Box::new(_c));
}
