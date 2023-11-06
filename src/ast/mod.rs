pub mod base_node;
pub mod binary_operator;
pub mod literal;
mod precedence;
mod numeric;

pub use base_node::BaseNode;
pub use binary_operator::BinaryOperator;
pub use literal::Literal;
use precedence::precedence;

pub struct Ast {
    pub root: Box<dyn BaseNode>,
}

impl Ast {
    pub fn new(expression: String) -> Self {
        let tokens = Self::tokenize(expression);
        let posfix_tokens = Self::infix_to_posfix(tokens);
        let root: Box<dyn BaseNode> = Self::build_ast(posfix_tokens).unwrap();
        Self { root }
    }

    fn tokenize(expression: String) -> Vec<String> {
        let tokens: Vec<String> = expression
            .replace(' ', "")
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let mut output: Vec<String> = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            let token = tokens[i].clone();
            if ["+", "-", "*", "/", "(", ")"].contains(&token.as_str()) {
                output.push(token);
            } else {
                let mut number = token.clone();
                let mut pos = 0;
                while let Some(next_token) = tokens.get(i + 1 + pos) {
                    if ["+", "-", "*", "/", "(", ")"].contains(&next_token.as_str()) {
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

    fn infix_to_posfix(tokens: Vec<String>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut stack: Vec<String> = Vec::new();

        for token in tokens {
            if token == "+" || token == "-" || token == "*" || token == "/" {
                while !stack.is_empty() && precedence(&stack[stack.len() - 1]) >= precedence(&token)
                {
                    output.push(stack.pop().unwrap());
                }
                stack.push(token);
            } else if token == "(" {
                stack.push(token);
            } else if token == ")" {
                while !stack.is_empty() && stack[stack.len() - 1] != "(" {
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

    pub fn build_ast(posfix_tokens: Vec<String>) -> Result<Box<dyn BaseNode>, String> {
        let mut stack: Vec<Box<dyn BaseNode>> = Vec::new();
        let operators = ["+", "-", "*", "/"];

        for token in posfix_tokens {
            if operators.contains(&token.as_str()) {
                let right = stack
                    .pop()
                    .ok_or("Stack underflow: no right operand".to_string())?;
                let left = stack
                    .pop()
                    .ok_or("Stack underflow: no left operand".to_string())?;
                let node = Box::new(BinaryOperator::new(left, right, token));
                stack.push(node);
            } else {
                let value = token
                    .parse::<f64>()
                    .map_err(|_| format!("Failed to parse token: {}", token))?;
                let node = Box::new(Literal::new(value));
                stack.push(node);
            }
        }

        stack.pop().ok_or("Stack underflow: no result".to_string())
    }
}
