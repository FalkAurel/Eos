use std::{collections::HashMap, fmt::Display};
use DynType::*;
use super::value::Value;

#[derive(Debug)]
pub enum DynType {
    Text(Box<ObjString>),
    HashMap(Box<HashMap<Value, Value>>)
}

impl DynType {
    pub fn from<T: Into<DynType>>(data: T) -> Self {
        data.into()
    }

    pub fn add(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Text(a), Text(b)) => Some(DynType::from(a.as_ref().data.to_owned() + b.as_ref().data.as_ref())),
            _ => None
        }
    }
}

impl From<String> for DynType {
    fn from(value: String) -> DynType {
        DynType::Text(Box::new(ObjString::new(value)))
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Text(ptr), Text(ptr1)) => ptr.as_ref() == ptr1.as_ref(),
            _ => false
        }
    }
}

impl Display for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text(ptr) => write!(f, "{}", &ptr.data),
            HashMap(ptr) => write!(f, "{:?}", ptr.as_ref())
        }
    }
}


#[derive(Debug)]
pub struct ObjString {
    data: String,
    hash: usize
}

impl ObjString {
    pub fn new(data: String) -> Self {
        let hash: usize = Self::compute_hash(&data);
        Self {data, hash}
    }

    fn compute_hash(string: &String) -> usize {
        let mut hash: usize = 2166136261;

        for element in string.as_bytes() {
            hash ^= *element as usize;
            hash = usize::wrapping_mul(hash, 16777619);
        }

        hash
    }
}

impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        // First, comparing the hashes. If they are different, the underlying data must be different, as the hash function is deterministic.
        if self.hash == other.hash {
            // Only if the hashes are equal, compare the actual data. This accounts for collisions.
            self.data == other.data
        } else {
            false
        }
    }
}
