use crate::error::ParserError;
use crate::node::ast_node::Context;
use crate::value::Value;
use std::fmt::{Debug, Formatter};

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        let value = context.get(&self.name.as_str());

        if let Some(value) = value {
            Ok(Value::Float(*value))
        } else {
            Err(ParserError::EvaluationError)
        }
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Variable")
            .field("name", &self.name)
            .finish()
    }
}
