mod ast;

use ast::Ast;

fn main() {
    let str = String::from("2 * (124-79)+17");
    let ast = Ast::new(str);

    println!("{}", ast);
}
