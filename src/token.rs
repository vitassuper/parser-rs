use std::fmt::Display;
use crate::operator::Operator;

#[derive(Debug, PartialEq)]
pub enum Token {
    Operator(Operator),
    Number(f64),
    Identifier(String),
    LParen,
    RParen,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Operator(_) => {
                write!(f, "Operator")
            }
            Token::Number(number) => {
                write!(f, "Number: {}", number)
            }
            Token::Identifier(name) => {
                write!(f, "Identifier: {}", name)
            }
            Token::LParen => {
                write!(f, "Token: (")
            }
            Token::RParen => {
                write!(f, "Token: )")
            }
        }
    }
}
