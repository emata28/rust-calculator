pub mod calc;

fn main() {
    let x = calc::process_equation();
    match x {
        Some(x) => {
            println!("{}", x);
        }
        None => {
            println!("NaN");
        }
    }
}
