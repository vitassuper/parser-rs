use crate::node::ast_node::AstNode;
use crate::operator::Operator;

pub trait UnaryExpression {
    fn new(operator: Operator, right: Box<dyn AstNode>) -> Self;
}
