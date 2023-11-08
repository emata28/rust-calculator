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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_operator() {
        assert!(is_operator(&"+".to_string()));
        assert!(is_operator(&"-".to_string()));
        assert!(is_operator(&"*".to_string()));
        assert!(is_operator(&"/".to_string()));
        assert!(is_operator(&"(".to_string()));
        assert!(is_operator(&")".to_string()));
        assert!(!is_operator(&"1".to_string()));
        assert!(!is_operator(&"a".to_string()));
        assert!(!is_operator(&" ".to_string()));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(precedence(&"+".to_string()), 1);
        assert_eq!(precedence(&"-".to_string()), 1);
        assert_eq!(precedence(&"*".to_string()), 2);
        assert_eq!(precedence(&"/".to_string()), 2);
        assert_eq!(precedence(&"(".to_string()), 0);
        assert_eq!(precedence(&")".to_string()), 0);
        assert_eq!(precedence(&"1".to_string()), 0);
        assert_eq!(precedence(&"a".to_string()), 0);
        assert_eq!(precedence(&" ".to_string()), 0);
    }

    #[test]
    fn test_is_left_parenthesis() {
        assert!(is_left_parenthesis(&"(".to_string()));
        assert!(!is_left_parenthesis(&")".to_string()));
        assert!(!is_left_parenthesis(&"+".to_string()));
        assert!(!is_left_parenthesis(&"-".to_string()));
        assert!(!is_left_parenthesis(&"*".to_string()));
        assert!(!is_left_parenthesis(&"/".to_string()));
        assert!(!is_left_parenthesis(&"1".to_string()));
        assert!(!is_left_parenthesis(&"a".to_string()));
        assert!(!is_left_parenthesis(&" ".to_string()));
    }

    #[test]
    fn test_is_right_parenthesis() {
        assert!(!is_right_parenthesis(&"(".to_string()));
        assert!(is_right_parenthesis(&")".to_string()));
        assert!(!is_right_parenthesis(&"+".to_string()));
        assert!(!is_right_parenthesis(&"-".to_string()));
        assert!(!is_right_parenthesis(&"*".to_string()));
        assert!(!is_right_parenthesis(&"/".to_string()));
        assert!(!is_right_parenthesis(&"1".to_string()));
        assert!(!is_right_parenthesis(&"a".to_string()));
        assert!(!is_right_parenthesis(&" ".to_string()));
    }

    #[test]
    fn test_is_math_operator() {
        assert!(is_math_operator(&"+".to_string()));
        assert!(is_math_operator(&"-".to_string()));
        assert!(is_math_operator(&"*".to_string()));
        assert!(is_math_operator(&"/".to_string()));
        assert!(!is_math_operator(&"(".to_string()));
        assert!(!is_math_operator(&")".to_string()));
        assert!(!is_math_operator(&"1".to_string()));
        assert!(!is_math_operator(&"a".to_string()));
        assert!(!is_math_operator(&" ".to_string()));
    }
}
