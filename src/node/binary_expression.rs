use crate::node::binary_arithmetic_expression::ArithmeticBinaryExpression;
use crate::node::binary_comparison_expression::ComparisonBinaryExpression;
use crate::node::binary_logical_expression::LogicalBinaryExpression;

#[derive(Debug)]
pub enum BinaryExpression {
    BinaryArithmeticExpression(ArithmeticBinaryExpression),
    BinaryComparisonExpression(ComparisonBinaryExpression),
    BinaryLogicalExpression(LogicalBinaryExpression),
}
