use crate::error::LexerError::{ConversionError, UnexpectedCharacter};
use crate::error::ParserError;
use crate::operator::ArithmeticOperator::{Div, Minus, Mul, Plus};
use crate::operator::ComparisonOperator::{
    Equals, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual,
};
use crate::operator::LogicalOperator::{And, Or};
use crate::operator::Operator;
use crate::token::Token;

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(raw_string: &str) -> Result<Vec<Token>, ParserError> {
        let mut tokens: Vec<Token> = vec![];
        let mut chars = raw_string.chars().peekable();

        while let Some(char) = chars.next() {
            if char.is_whitespace() {
                continue;
            }

            match char {
                '+' => {
                    tokens.push(Token::Operator(Operator::ArithmeticOperator(Plus)));
                }
                '*' => {
                    tokens.push(Token::Operator(Operator::ArithmeticOperator(Mul)));
                }
                '/' => {
                    tokens.push(Token::Operator(Operator::ArithmeticOperator(Div)));
                }
                '-' => {
                    tokens.push(Token::Operator(Operator::ArithmeticOperator(Minus)));
                }
                '>' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::ComparisonOperator(
                            GreaterThanOrEqual,
                        )));
                    } else {
                        tokens.push(Token::Operator(Operator::ComparisonOperator(GreaterThan)));
                    }
                }
                '<' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::ComparisonOperator(
                            LessThanOrEqual,
                        )));
                    } else {
                        tokens.push(Token::Operator(Operator::ComparisonOperator(LessThan)));
                    }
                }
                '=' => {
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::Operator(Operator::ComparisonOperator(Equals)));
                    } else {
                        return Err(ParserError::LexerError(UnexpectedCharacter('=')));
                    }
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut var_name = char.to_string();
                    while let Some(next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || *next_char == '_' {
                            var_name.push(*next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(match var_name.as_str() {
                        "and" => Token::Operator(Operator::LogicalOperator(And)),
                        "or" => Token::Operator(Operator::LogicalOperator(Or)),
                        _ => Token::Identifier(var_name),
                    })
                }
                '0'..='9' => {
                    let mut number_str = char.to_string();
                    while let Some(next_char) = chars.peek() {
                        if next_char.is_numeric() || *next_char == '.' {
                            number_str.push(*next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let number = number_str
                        .parse::<f64>()
                        .map_err(|_| ParserError::LexerError(ConversionError(number_str)))?;

                    tokens.push(Token::Number(number));
                }
                '(' => {
                    tokens.push(Token::LParen);
                }
                ')' => {
                    tokens.push(Token::RParen);
                }
                _ => return Err(ParserError::LexerError(UnexpectedCharacter(char))),
            }
        }

        Ok(tokens)
    }
}
