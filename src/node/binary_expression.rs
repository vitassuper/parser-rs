use crate::node::ast_node::AstNode;
use crate::operator::Operator;

pub trait BinaryExpression: AstNode {
    fn new(left: Box<dyn AstNode>, operator: Operator, right: Box<dyn AstNode>) -> Self
    where
        Self: Sized;
}
