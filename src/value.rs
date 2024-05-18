use std::{fmt::{Debug, Display}, ops::{Add, Div, Mul, Sub}};
use Value::*;

type Error = String;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    ObjString(Box<String>),
    Null,
}

pub trait Negate {
    type Output;
    fn negate(self) -> Self::Output;
}

impl Add for Value {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Integer(a), Integer(b)) => Ok(Integer(a + b)),
            (Integer(a), Float(b)) => Ok(Float(*a as f64 + b)),
            (Float(a), Integer(b)) => Ok(Float(a + *b as f64)),
            (Float(a), Float(b)) => Ok(Float(a + b)),
            _ => Err(format!("{:?} and {:?} can not be added", self, rhs))
        }
    }
}

impl Sub for Value {
    type Output = Result<Self, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Integer(a), Integer(b)) => Ok(Integer(a - b)),
            (Integer(a), Float(b)) => Ok(Float(*a as f64 - b)),
            (Float(a), Integer(b)) => Ok(Float(a - *b as f64)),
            (Float(a), Float(b)) => Ok(Float(a - b)),
            _ => Err(format!("{:?} and {:?} can not be subtracted", self, rhs))
        }
    }
}

impl Mul for Value {
    type Output = Result<Self, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Integer(a), Integer(b)) => Ok(Integer(a * b)),
            (Integer(a), Float(b)) => Ok(Float(*a as f64 * b)),
            (Float(a), Integer(b)) => Ok(Float(a * *b as f64)),
            (Float(a), Float(b)) => Ok(Float(a * b)),
            _ => Err(format!("{:?} and {:?} can not be multiplied", self, rhs))
        }
    }
}

impl Div for Value {
    type Output = Result<Self, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Integer(a), Integer(b)) => Ok(Integer(a / b)),
            (Integer(a), Float(b)) => Ok(Float(*a as f64 / b)),
            (Float(a), Integer(b)) => Ok(Float(a / *b as f64)),
            (Float(a), Float(b)) => Ok(Float(a / b)),
            _ => Err(format!("{:?} and {:?} can not be divided", self, rhs))
        }
    }
}


pub trait Comparison {
    fn greater(&self, other: &Self) -> Result<Value, Error>;

    fn less(&self, other: &Self) -> Result<Value, Error>;
}

impl Comparison for Value {
    fn greater(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Integer(a), Integer(b)) => Ok(Boolean(a > b)),
            (Integer(a), Float(b)) => Ok(Boolean(*a as f64 > *b)),
            (Float(a), Integer(b)) => Ok(Boolean(*a > *b as f64)),
            (Float(a), Float(b)) => Ok(Boolean(a > b)),
            _ => Err(" > is only available for Float and Integer".to_string())
        }
    }

    fn less(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Integer(a), Integer(b)) => Ok(Boolean(a < b)),
            (Integer(a), Float(b)) => Ok(Boolean((*a as f64) < *b)),
            (Float(a), Integer(b)) => Ok(Boolean(*a < *b as f64)),
            (Float(a), Float(b)) => Ok(Boolean(a > b)),
            _ => Err(" > is only available for Float and Integer".to_string())
        }
    }
}

impl Negate for Value {
    type Output = Result<Value, Error>;
    fn negate(self) -> Self::Output {
        match self {
            Integer(a) => Ok(Integer(-a)),
            Float(a) => Ok(Float(-a)),
            Boolean(a) => Ok(Boolean(!a)),
            _ => Err(format!("{:?} CANNOT BE NEGATED", self))
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}", value),
            Boolean(value) => write!(f, "{}", value),
            Null => write!(f, "Null"),
            ObjString(value) => {
                write!(f, "{}", value.as_ref())
            }
        }
    }
}
