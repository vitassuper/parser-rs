#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    ArithmeticOperator(ArithmeticOperator),
    ComparisonOperator(ComparisonOperator),
    LogicalOperator(LogicalOperator),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ArithmeticOperator {
    Plus,
    Minus,
    Mul,
    Div,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum ArithmeticOperator {
//     BinaryArithmeticOperator(BinaryArithmeticOperator),
//     UnaryArithmeticOperator(UnaryArithmeticOperator)
// }

// #[derive(Clone, Debug, PartialEq)]
// pub enum BinaryArithmeticOperator {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }
//
// #[derive(Clone, Debug, PartialEq)]
// pub enum UnaryArithmeticOperator {
//     Negate
// }

#[derive(Debug, PartialEq, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    Equals,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
