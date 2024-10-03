use crate::error::ParserError;
use crate::node::ast_node::{AstNode, Context};
use crate::operator::ArithmeticOperator;
use crate::value::Value;

#[derive(Debug)]
pub struct ArithmeticBinaryExpression {
    left: Box<AstNode>,
    operator: ArithmeticOperator,
    right: Box<AstNode>,
}

impl ArithmeticBinaryExpression {
    pub fn new(left: AstNode, operator: ArithmeticOperator, right: AstNode) -> Self
    where
        Self: Sized,
    {
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
            ArithmeticOperator::Plus => left_value + right_value,
            ArithmeticOperator::Minus => left_value - right_value,
            ArithmeticOperator::Mul => left_value * right_value,
            ArithmeticOperator::Div => left_value / right_value,
        };

        result.ok_or(ParserError::EvaluationError)
    }
}
