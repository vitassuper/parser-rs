use crate::error::ParserError;
use crate::value::Value;
use std::collections::HashMap;

pub type Context<'a> = HashMap<&'a str, f64>;

pub trait AstNode: std::fmt::Debug {
    fn evaluate(&self, context: &Context) -> Result<Value, ParserError>;
}
