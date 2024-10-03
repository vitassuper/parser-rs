use crate::node::unary_arithmetic_expression::ArithmeticUnaryExpression;

#[derive(Debug)]
pub enum UnaryExpression {
    UnaryArithmeticExpression(ArithmeticUnaryExpression),
}
