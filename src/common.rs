use std::{fmt::Debug, ptr};

pub static DEBUG_BYTECODE: bool = false;
pub const DEFAULT_STACK_CAPACITY: usize = 1024;
pub static ENABLE_TESTING: bool = true;


pub fn compile_error(msg: &str){
    eprintln!("[COMPILE ERROR] {msg}")
}

pub fn runtime_error<T: Debug>(msg: T){
    eprintln!("[RUNTIME ERROR]: {msg:?}")
}


//this is just a container to avoid conflicts with the borrow checker and it's only the size of a reference
pub struct SharedData<T>{
    data: *const T
}

impl <T> SharedData<T> {
    pub fn new(data: &T) -> Self{
        Self { data:  ptr::addr_of!(*data)}
    }
}

impl <T>AsRef<T> for SharedData<T> {
    fn as_ref(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }
}

impl <T> AsMut<T> for SharedData<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe {
            self.data.cast_mut().as_mut().unwrap()
        }
    }
}
