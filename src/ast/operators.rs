pub fn is_operator(c: &String) -> bool {
    OPERATORS.contains(&c.as_str())
}

pub fn precedence(c: &String) -> i32 {
    let c = c.as_str();
    match c {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

pub fn is_left_parenthesis(c: &String) -> bool {
    c == "("
}

pub fn is_right_parenthesis(c: &String) -> bool {
    c == ")"
}

pub fn is_math_operator(c: &String) -> bool {
    c == "+" || c == "-" || c == "*" || c == "/"
}

const OPERATORS: [&str; 6] = ["+", "-", "*", "/", "(", ")"];
