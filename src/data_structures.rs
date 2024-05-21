use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::OnceLock;
use super::value::Error;

use DynType::*;
use obj_string::ObjString;


// Static mutable global state for interned strings, initialized once.
static mut INTERNED_STRINGS: OnceLock<HashMap<usize, Rc<ObjString>>> = OnceLock::new();

pub fn initialize_interned_string() {
    unsafe {
        INTERNED_STRINGS.get_or_init(|| HashMap::new());
    }
}

#[derive(Debug)]
pub enum DynType {
    Text(Rc<ObjString>),
}

impl DynType {
    pub fn from<T: Into<DynType>>(data: T) -> Self {
        data.into()
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Text(a), Text(b)) => {
                let (a, b) = (a.as_ref(), b.as_ref());
                Ok(Text(Rc::new(a.add(b))))
            },
            //_ => Err(format!("{:?} and {:?} can not be added", self, other))
        }
    }
}

impl From<String> for DynType {
    fn from(value: String) -> DynType {
        let hash: usize = ObjString::compute_hash(&value);

        unsafe {
            let interned_string: &mut HashMap<usize, Rc<ObjString>> = INTERNED_STRINGS.get_mut()
                .expect("Use 'initialize_interned_string' in main to intialize INTERNED_STRING");

            if let Some(entry) = interned_string.get(&hash) {
                let copy: Rc<ObjString> = entry.clone();
                return DynType::Text(copy);
            }

            let entry: Rc<ObjString> = Rc::new(ObjString::new(value));
            interned_string.insert(hash, entry.clone());

            DynType::Text(entry)
        }
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Text(ptr), Text(ptr1)) => *ptr == *ptr1 // or use Arc::ptr_eq(ptr, ptr1)
            //_ => false
        }
    }
}

impl Display for DynType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Text(ptr) => write!(f, "{}", ptr.get_data())
        }
    }
}


pub mod obj_string {

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

        pub fn add(&self, other: &Self) -> Self {
            let temp: String = self.data.to_owned() + other.data.as_ref();
            Self::new(temp)
        }

        pub fn get_data(&self) -> &String {
            &self.data
        }

        pub fn compute_hash(string: &String) -> usize {
            let offset_bias: usize = 14695981039346656037;
            let fnv_prime: usize = 1099511628211;

            string.as_bytes().iter().fold(offset_bias, move |mut hash: usize, element: &u8| -> usize {
                hash ^= *element as usize;
                hash.wrapping_mul(fnv_prime)
            })
        }
    }

    impl PartialEq for ObjString {
        fn eq(&self, other: &Self) -> bool {
            // Using short-circuit evaluation to first only compare the hashes and only if the hashes match we compare the undelying data
            self.hash == other.hash && self.data == other.data
        }
    }
}
