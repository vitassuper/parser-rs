use crate::error::ParserError;
use crate::node::ast_node::{AstNode, Context};
use crate::operator::LogicalOperator;
use crate::value::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct LogicalBinaryExpression {
    left: Box<AstNode>,
    operator: LogicalOperator,
    right: Box<AstNode>,
}

impl LogicalBinaryExpression {
    pub fn new(left: AstNode, operator: LogicalOperator, right: AstNode) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        let left_value = self.left.evaluate(context)?;
        let right_value = self.right.evaluate(context)?;

        let result = match self.operator {
            LogicalOperator::And => match (left_value, right_value) {
                (Value::Bool(left_boolean), Value::Bool(right_boolean)) => {
                    Some(Value::Bool(left_boolean && right_boolean))
                }
                _ => None,
            },

            LogicalOperator::Or => match (left_value, right_value) {
                (Value::Bool(left_boolean), Value::Bool(right_boolean)) => {
                    Some(Value::Bool(left_boolean || right_boolean))
                }
                _ => None,
            },
        };

        result.ok_or(ParserError::EvaluationError)
    }
}
