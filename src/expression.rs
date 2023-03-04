
use crate::scanner::{Token, TokenType};

#[derive(Debug)]
pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn to_string(self: &Self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Grouping { expr: Box<Expression> },
    Literal { value: LiteralValue },
    Unary { operator: Token, right: Box<Expression> },
}

impl Expression {
    pub fn to_string(self: &Self) -> String {
        match self {
            Expression::Binary { left, operator, right } => format!("{} {} {}", &operator.lexeme, (*left).to_string(), (*right).to_string()),
            Expression::Grouping { expr } => format!("(group {})", expr.to_string()),
            Expression::Literal { value } => format!("{}", value.to_string()),
            Expression::Unary { operator, right } => {
                let operator_str = &operator.lexeme;
                let right_str = (*right).to_string();
                format!("{} {}", operator_str, right_str)
            },
        }
    }

    pub fn print(self: &Self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Expression::*;
    use super::LiteralValue::*;

    #[test]
    fn handle_pretty_print() {
        let minus = Token{ 
            token_type: TokenType::Minus, lexeme: "-".to_string(), 
            literal: None, 
            line_number: 0 
        };
        let num = Literal{ 
            value: Number(123.0) 
        };
        let multi = Token{ 
            token_type: TokenType::Star, lexeme: "*".to_string(), 
            literal: None, line_number: 0 
        };
        let group = Grouping{ 
            expr: Box::from(
                Literal { 
                    value: Number(45.67) 
                }
            )
        };

        let ast = Binary{ 
            left: Box::from(
                Unary
                { 
                    operator: minus, right: Box::from(num) 
                }
            ),
            operator: multi,
            right: Box::from(group),
        };

        ast.print();
    }
}