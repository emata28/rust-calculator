pub mod base_node;
pub mod binary_operator;
pub mod literal;
mod numeric;
mod operators;

pub use base_node::BaseNode;
pub use binary_operator::BinaryOperator;
pub use literal::Literal;
use numeric::Numeric;
use operators as Operators;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Ast {
    pub root: Box<dyn BaseNode>,
}

impl Ast {
    pub fn new(expression: String) -> Self {
        let tokens = Self::tokenize(expression);
        let postfix_tokens = Self::infix_to_postfix(tokens);
        let root: Box<dyn BaseNode> = Self::build_ast(postfix_tokens).unwrap();
        Self { root }
    }

    fn tokenize(expression: String) -> Vec<String> {
        let tokens: Vec<String> = expression
            .replace(' ', "")
            .split("")
            .filter(|s| {
                Operators::is_operator(&s.to_string()) || Literal::is_numeric(s.to_string())
            })
            .map(|s| s.to_string())
            .collect();

        let mut output: Vec<String> = Vec::new();
        let mut i = 0;
        while let Some(token) = tokens.get(i) {
            if Operators::is_operator(token) {
                output.push(tokens[i].clone());
            } else {
                let mut number = tokens[i].clone();
                let mut pos = 0;
                while let Some(next_token) = tokens.get(i + 1 + pos) {
                    if Operators::is_operator(next_token) {
                        break;
                    }
                    number.push_str(next_token);
                    pos += 1;
                }
                i += pos;
                output.push(number);
            }
            i += 1;
        }
        output
    }

    fn infix_to_postfix(tokens: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut stack: Vec<String> = Vec::new();

        for token in tokens {
            if Operators::is_math_operator(&token) {
                while !stack.is_empty()
                    && Operators::precedence(&stack[stack.len() - 1])
                        >= Operators::precedence(&token)
                {
                    output.push(stack.pop().unwrap());
                }
                stack.push(token);
            } else if Operators::is_left_parenthesis(&token) {
                stack.push(token);
            } else if Operators::is_right_parenthesis(&token) {
                while !stack.is_empty() && !Operators::is_left_parenthesis(&stack[stack.len() - 1])
                {
                    output.push(stack.pop().unwrap());
                }
                stack.pop();
            } else {
                output.push(token);
            }
        }

        while let Some(token) = stack.pop() {
            output.push(token);
        }

        output
    }

    fn build_ast(postfix_tokens: Vec<String>) -> Result<Box<dyn BaseNode>, String> {
        let mut stack: Vec<Box<dyn BaseNode>> = Vec::new();

        for token in postfix_tokens {
            if Operators::is_operator(&token) {
                let right = stack
                    .pop()
                    .ok_or_else(|| "Stack underflow: no right operand".to_string())?;
                let left = stack
                    .pop()
                    .ok_or_else(|| "Stack underflow: no left operand".to_string())?;
                let node = Box::new(BinaryOperator::new(left, right, token));
                stack.push(node);
            } else {
                let value = token
                    .parse::<Numeric>()
                    .expect("Failed to parse token to Numeric");
                let node = Box::new(Literal::new(value));
                stack.push(node);
            }
        }

        stack
            .pop()
            .ok_or_else(|| "Stack underflow: no result".to_string())
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let expression = "1+2*3-4/5".to_string();
        let tokens = Ast::tokenize(expression);
        assert_eq!(tokens, vec!["1", "+", "2", "*", "3", "-", "4", "/", "5"]);
    }

    #[test]
    fn test_tokenize_with_parenthesis() {
        let expression = "(1+2)*3-(4/5)".to_string();
        let tokens = Ast::tokenize(expression);
        assert_eq!(
            tokens,
            vec!["(", "1", "+", "2", ")", "*", "3", "-", "(", "4", "/", "5", ")"]
        );
    }

    #[test]
    fn test_tokenize_with_spaces() {
        let expression = "1 + 2 * 3 - 4 / 5".to_string();
        let tokens = Ast::tokenize(expression);
        assert_eq!(tokens, vec!["1", "+", "2", "*", "3", "-", "4", "/", "5"]);
    }

    #[test]
    fn test_tokenize_with_spaces_and_parenthesis() {
        let expression = "( 1 + 2 ) * 3 - ( 4 / 5 )".to_string();
        let tokens = Ast::tokenize(expression);
        assert_eq!(
            tokens,
            vec!["(", "1", "+", "2", ")", "*", "3", "-", "(", "4", "/", "5", ")"]
        );
    }

    #[test]
    fn test_tokenize_wrong_expression() {
        let expression = "1+2*3-4/5a".to_string();
        let tokens = Ast::tokenize(expression);
        assert_eq!(tokens, vec!["1", "+", "2", "*", "3", "-", "4", "/", "5"]);
    }

    #[test]
    fn test_infix_to_postfix() {
        let tokens = vec!["1", "+", "2", "*", "3", "-", "4", "/", "5"];
        let postfix_tokens = Ast::infix_to_postfix(tokens.iter().map(|s| s.to_string()).collect());
        assert_eq!(
            postfix_tokens,
            vec!["1", "2", "3", "*", "+", "4", "5", "/", "-"]
        );
    }

    #[test]
    fn test_infix_to_postfix_with_parenthesis() {
        let tokens = vec![
            "(", "1", "+", "2", ")", "*", "3", "-", "(", "4", "/", "5", ")",
        ];
        let postfix_tokens = Ast::infix_to_postfix(tokens.iter().map(|s| s.to_string()).collect());
        assert_eq!(
            postfix_tokens,
            vec!["1", "2", "+", "3", "*", "4", "5", "/", "-"]
        );
    }

    #[test]
    fn test_infix_to_postfix_with_spaces() {
        let expression = "1 + 2 * 3 - 4 / 5".to_string();
        let tokens = Ast::tokenize(expression);
        let postfix_tokens = Ast::infix_to_postfix(tokens.iter().map(|s| s.to_string()).collect());
        assert_eq!(
            postfix_tokens,
            vec!["1", "2", "3", "*", "+", "4", "5", "/", "-"]
        );
    }

    #[test]
    fn test_infix_to_postfix_with_spaces_and_parenthesis() {
        let expression = "( 1 + 2 ) * 3 - ( 4 / 5 )".to_string();
        let tokens = Ast::tokenize(expression);
        let postfix_tokens = Ast::infix_to_postfix(tokens.iter().map(|s| s.to_string()).collect());
        assert_eq!(
            postfix_tokens,
            vec!["1", "2", "+", "3", "*", "4", "5", "/", "-"]
        );
    }

    #[test]
    fn test_build_ast() {
        let expression = "1+2*3-4/5".to_string();
        let tokens = Ast::tokenize(expression);
        let postfix_tokens = Ast::infix_to_postfix(tokens.iter().map(|s| s.to_string()).collect());
        let ast = Ast::build_ast(postfix_tokens.iter().map(|s| s.to_string()).collect()).unwrap();
        assert_eq!(ast.evaluate(), Numeric::Integer(7));
    }
}
