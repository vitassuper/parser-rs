use crate::error::ParserError;
use crate::node::arithmetic_binary_expression::ArithmeticBinaryExpression;
use crate::node::arithmetic_unary_expression::ArithmeticUnaryExpression;
use crate::node::ast_node::AstNode;
use crate::node::comparison_binary_expression::ComparisonBinaryExpression;
use crate::node::literal::Literal;
use crate::node::logical_binary_expression::LogicalBinaryExpression;
use crate::node::variable::Variable;
use crate::operator::{ArithmeticOperator, Operator};
use crate::token::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(
        &mut self,
        min_precedence: u16,
    ) -> Result<Box<dyn AstNode>, ParserError> {
        let mut node = self.parse_primary();

        loop {
            let token = self.tokens.peek();

            if let Some(Token::Operator(operator)) = token {
                let precedence = Parser::get_operator_precedence(operator);

                if precedence < min_precedence {
                    break;
                }

                let op = operator.clone();

                self.consume_token()?;

                node = Ok(Parser::get_binary_expression(
                    node?,
                    op,
                    self.parse_binary_expression(precedence + 1)?,
                ));
            } else {
                break;
            }
        }

        node
    }

    fn get_binary_expression(
        left: Box<dyn AstNode>,
        operator: Operator,
        right: Box<dyn AstNode>,
    ) -> Box<dyn AstNode> {
        match operator {
            Operator::ArithmeticOperator(op) => {
                Box::new(ArithmeticBinaryExpression::new(left, op, right))
            }

            Operator::ComparisonOperator(op) => {
                Box::new(ComparisonBinaryExpression::new(left, op, right))
            }

            Operator::LogicalOperator(op) => {
                Box::new(LogicalBinaryExpression::new(left, op, right))
            }
        }
    }

    fn consume_token(&mut self) -> Result<Token, ParserError> {
        self.tokens.next().ok_or(ParserError::UnexpectedEndOfInput)
    }

    fn parse_primary(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        let token = self.consume_token()?;

        match token {
            Token::Operator(Operator::ArithmeticOperator(ArithmeticOperator::Minus)) => {
                Ok(Box::new(ArithmeticUnaryExpression::new(
                    ArithmeticOperator::Minus,
                    self.parse_primary()?,
                )))
            }

            Token::Identifier(name) => Ok(Box::new(Variable::new(name))),
            Token::Number(number) => Ok(Box::new(Literal::new(number))),
            Token::LParen => {
                let expr = self.parse()?;
                self.consume_token()?;

                Ok(expr)
            }
            _ => Err(ParserError::UnexpectedTokenError(token)),
        }
    }

    fn get_operator_precedence(operator: &Operator) -> u16 {
        match operator {
            Operator::ArithmeticOperator(arithmetic_operator) => match arithmetic_operator {
                ArithmeticOperator::Mul => 4,
                ArithmeticOperator::Div => 4,
                ArithmeticOperator::Plus => 3,
                ArithmeticOperator::Minus => 3,
            },
            Operator::ComparisonOperator(_) => 2,
            Operator::LogicalOperator(_) => 1,
        }
    }
}
