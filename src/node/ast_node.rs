use crate::error::ParserError;
use crate::node::binary_expression::BinaryExpression;
use crate::node::literal::Literal;
use crate::node::unary_expression::UnaryExpression;
use crate::node::variable::Variable;
use crate::value::Value;
use std::collections::HashMap;

pub type Context<'a> = HashMap<&'a str, f64>;

#[derive(Debug)]
pub enum AstNode {
    Variable(Variable),
    Literal(Literal),
    UnaryExpression(UnaryExpression),
    BinaryExpression(BinaryExpression),
}

impl AstNode {
    pub fn evaluate(&self, context: &Context) -> Result<Value, ParserError> {
        match self {
            AstNode::Variable(var) => var.evaluate(context),
            AstNode::Literal(lit) => lit.evaluate(context),
            AstNode::UnaryExpression(unary_expr) => match unary_expr {
                UnaryExpression::UnaryArithmeticExpression(expr) => expr.evaluate(context),
            },
            AstNode::BinaryExpression(binary_expr) => match binary_expr {
                BinaryExpression::BinaryArithmeticExpression(expr) => expr.evaluate(context),
                BinaryExpression::BinaryComparisonExpression(expr) => expr.evaluate(context),
                BinaryExpression::BinaryLogicalExpression(expr) => expr.evaluate(context),
            },
        }
    }
}
