pub fn precedence(operator: &str) -> i32 {
    match operator {
        "+" => 1,
        "-" => 1,
        "*" => 2,
        "/" => 2,
        "(" => 0,
        ")" => 0,
        _ => panic!("Unknown operator {}", operator),
    }
}
