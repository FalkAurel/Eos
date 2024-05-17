use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;
use std::ptr::{copy_nonoverlapping, read, write};


#[derive(Debug)]
pub struct ArrayList {
    data: *mut u8,
    size: usize,
    capacity: usize
}

impl ArrayList {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            let stack: *mut u8 = alloc(Layout::array::<u8>(capacity).unwrap());
            Self { data: stack, size: 0, capacity }
        }
    }

    pub fn push<T>(&mut self, data: T){
        while size_of::<T>() + self.size >= self.capacity {
            self.resize();
        }

        unsafe{
            write(self.data.add(self.size) as *mut T, data)
        }

        self.size += size_of::<T>();
    }

    pub fn pop<T>(&mut self) -> Option<T> {
        if  self.size == 0 {
            return None;
        }

        unsafe {
            let temp: Option<T> = Some(read(self.data.add(self.size) as *const T));

            self.size -= 1;

            temp
        }
    }

    pub fn peek<T>(&self, index: usize) -> Option<T> {
        if index + size_of::<T>() >= self.size {
            return None;
        }

        unsafe {
            Some(read(self.data.add(index) as *const T))
        }
    }

    pub fn get_content<T>(&self) -> Vec<T>{
        let mut counter: usize = 0;
        let mut vector: Vec<T> = vec![];

        while let Some(element) = self.peek::<T>(counter) {
            vector.push(element);
        }
        vector
    }

    fn resize(&mut self) {
        let new_capacity: usize = self.capacity * 2;
        let old_layout: Layout = Layout::array::<u8>(self.capacity).unwrap();
        let new_layout: Layout = Layout::array::<u8>(self.capacity).unwrap();

        unsafe {
            let temp: *mut u8 = alloc(new_layout);
            copy_nonoverlapping(self.data, temp, self.size);
            dealloc(self.data as *mut u8, old_layout);
            self.data = temp;
        }

        self.capacity = new_capacity;
    }
}

impl Drop for ArrayList {
    fn drop(&mut self) {
        unsafe {
            let old_layout: Layout = Layout::array::<u8>(self.capacity).unwrap();
            dealloc(self.data as *mut u8, old_layout);
        }
    }
}
