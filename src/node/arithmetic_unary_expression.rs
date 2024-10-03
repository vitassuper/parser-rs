use crate::error::ParserError;
use crate::node::ast_node::{AstNode, Context};
use crate::operator::ArithmeticOperator;
use crate::value::Value;
use std::fmt::Debug;

#[derive(Debug)]
pub struct ArithmeticUnaryExpression {
    operator: ArithmeticOperator,
    right: Box<dyn AstNode>,
}

// impl UnaryExpression for ArithmeticUnaryExpression {
impl ArithmeticUnaryExpression {
    pub fn new(operator: ArithmeticOperator, right: Box<dyn AstNode>) -> Self {
        Self { operator, right }
    }
}

impl AstNode for ArithmeticUnaryExpression {
    fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        let right_value = self.right.evaluate(context)?;

        let result = match self.operator {
            ArithmeticOperator::Minus => -right_value,
            _ => None,
        };

        result.ok_or(ParserError::EvaluationError)
    }
}
