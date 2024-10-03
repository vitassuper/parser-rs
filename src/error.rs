use crate::token::Token;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ParserError {
    LexerError(LexerError),
    EvaluationError,
    UnexpectedTokenError(Token),
    UnexpectedEndOfInput,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char),
    ConversionError(String),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedTokenError(token) => write!(f, "Unexpected token {}", token),
            ParserError::LexerError(err) => write!(f, "Lexer error: {}", err),
            ParserError::EvaluationError => write!(f, "Evaluation error"),
            ParserError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(msg) => write!(f, "Unexpected character: {}", msg),
            LexerError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
        }
    }
}
