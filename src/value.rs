use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Float(f64),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            _ => false, // Different variants are not equal
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            _ => None,
        }
    }
}

impl Mul for Value {
    type Output = Option<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a / b)),
            _ => None,
        }
    }
}

impl Neg for Value {
    type Output = Option<Value>;

    fn neg(self) -> Self::Output {
        if let Value::Float(neg) = self {
            Some(Value::Float(-neg))
        } else {
            None
        }
    }
}
