use std::collections::HashMap;
use std::fmt::Display;
use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

use super::value::Error;
use std::sync::Arc;

use DynType::*;
use obj_string::ObjString;

// I hate this so much, why can't I just use Rc. Arc has too much overhead and I don't even use the atomic feature
static INTERNED_STRING: Lazy<Mutex<HashMap<usize, Arc<ObjString>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[derive(Debug)]
pub enum DynType {
    Text(Arc<ObjString>),
}

impl DynType {
    pub fn from<T: Into<DynType>>(data: T) -> Self {
        data.into()
    }

    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        match (self, other) {
            (Text(a), Text(b)) => {
                let (a, b) = (a.as_ref(), b.as_ref());
                Ok(Text(Arc::new(a.add(b))))
            },
            //_ => Err(format!("{:?} and {:?} can not be added", self, other))
        }
    }
}

impl From<String> for DynType {
    fn from(value: String) -> DynType {
        let hash: usize = ObjString::compute_hash(&value);

        /*
        Forgive me for using such a janky implementation, usually I don't use global stuff.
        This time globally available stuff is the only feasable solution to contain the information needed to this file.
        Otherwisee we'd have to give the compiler knowledge of a this hashmap, this would also render our uniform DynType::new() function
        useless, as the from trait doesn't allow for any extensions.
        */

        let mut interned_strings: MutexGuard<HashMap<usize, Arc<ObjString>>>  = INTERNED_STRING.lock().expect("Expected a global Hashmap to intern the Strings");

        if interned_strings.contains_key(&hash) {
            let item: &Arc<ObjString> = interned_strings.get(&hash).unwrap();
            let copy: Arc<ObjString> = item.clone();

            return DynType::Text(copy);
        }

        let entry: Arc<ObjString> = Arc::new(ObjString::new(value));
        interned_strings.insert(hash, entry);
        let temp: Arc<ObjString> = interned_strings.get(&hash).unwrap().clone();

        DynType::Text(temp)
    }
}

impl PartialEq for DynType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Text(ptr), Text(ptr1)) => ptr == ptr1 // or use Arc::ptr_eq(ptr, ptr1)
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
    use std::hash::Hash;


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
            // First, comparing the hashes. If they are different, the underlying data must be different, as the hash function is deterministic.
            if self.hash == other.hash {
                // Only if the hashes are equal, compare the actual data. This accounts for collisions.
                self.data == other.data
            } else {
                false
            }
        }
    }

    impl Hash for ObjString {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {

        }
    }
}
