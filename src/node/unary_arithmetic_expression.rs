use crate::error::ParserError;
use crate::node::ast_node::{AstNode, Context};
use crate::operator::ArithmeticOperator;
use crate::value::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ArithmeticUnaryExpression {
    operator: ArithmeticOperator,
    right: Box<AstNode>,
}

impl ArithmeticUnaryExpression {
    pub fn new(operator: ArithmeticOperator, right: AstNode) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        let right_value = self.right.evaluate(context)?;

        let result = match self.operator {
            ArithmeticOperator::Minus => -right_value,
            _ => None,
        };

        result.ok_or(ParserError::EvaluationError)
    }
}
