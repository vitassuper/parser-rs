use crate::error::ParserError;
use crate::node::ast_node::{AstNode, Context};
use crate::operator::ComparisonOperator;
use crate::value::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ComparisonBinaryExpression {
    left: Box<dyn AstNode>,
    operator: ComparisonOperator,
    right: Box<dyn AstNode>,
}

// impl BinaryExpression for ComparisonBinaryExpression {
impl ComparisonBinaryExpression {
    pub fn new(
        left: Box<dyn AstNode>,
        operator: ComparisonOperator,
        right: Box<dyn AstNode>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl AstNode for ComparisonBinaryExpression {
    fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        let left_value = self.left.evaluate(context)?;
        let right_value = self.right.evaluate(context)?;

        if let (Value::Float(a), Value::Float(b)) = (left_value, right_value) {
            let result = match self.operator {
                ComparisonOperator::GreaterThanOrEqual => a >= b,
                ComparisonOperator::LessThanOrEqual => a <= b,
                ComparisonOperator::GreaterThan => a > b,
                ComparisonOperator::LessThan => a < b,
                ComparisonOperator::Equals => a == b,
            };

            Ok(Value::Bool(result))
        } else {
            Err(ParserError::EvaluationError)
        }
    }
}
