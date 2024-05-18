use std::fmt::Display;
use std::ops::Add;

use super::value::Value;
use DynType::*;

#[derive(Debug, Clone)]
pub enum DynType {
    Text(Box<String>)
}

impl DynType {
    pub fn new<T: Into<DynType>>(data: T) -> Self {
        data.into()
    }
}

impl Into<DynType> for String {
    fn into(self) -> DynType {
        DynType::Text(Box::new(self))
    }
}

impl Add for DynType{
    type Output = Option<Value>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Text(ptr1), Text(ptr2)) => Some(Value::Object(DynType::new(ptr1.as_ref().to_owned() + ptr2.as_ref())))
        }
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Text(ptr), Text(ptr1)) => ptr.as_ref() == ptr1.as_ref()
        }
    }
}

impl Display for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text(ptr) => write!(f, "{}", ptr.as_ref())
        }
    }
}
