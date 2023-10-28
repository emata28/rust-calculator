mod ast;
use ast::{binary_operator::BinaryOperator, literal::Literal};
pub mod calc;

fn main() {
    let a = Literal::<i16>::new(1);
    let b = Literal::<i16>::new(2);
    let op = BinaryOperator::new(Box::new(a), Box::new(b), String::from("+"));

    println!("{}", op.eval());
}
