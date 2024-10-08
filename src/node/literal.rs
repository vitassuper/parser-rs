use crate::error::ParserError;
use crate::node::ast_node::Context;
use crate::value::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Literal {
    number: f64,
}

impl Literal {
    pub fn new(number: f64) -> Self {
        Self { number }
    }

    pub fn evaluate(&self, _context: &Context) -> Result<Value, ParserError> {
        Ok(Value::Float(self.number))
    }
}
