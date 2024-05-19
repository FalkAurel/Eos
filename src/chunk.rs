use std::alloc::{alloc, dealloc, Layout};
use std::mem::size_of;
use std::ptr::{copy_nonoverlapping, read, write};

use super::opcode::OpCode::{self, *};
use super::value::Value;

const OPCODE_SIZE: usize = size_of::<OpCode>();
pub const VALUE_SIZE: usize = size_of::<Value>();
const LINE_SIZE: usize = size_of::<u32>();
pub const INSTRUCTION_SIZE: usize = OPCODE_SIZE + LINE_SIZE;


/* MEMORY LAYOUT

 All the opcodes, lines and values are stored in a one long arraylist.
 When we store a Value we indicate this by storing OpCode::Constant first. Which is followed by the line.

  CONSTANT LINE VALUE

 VALUE will read the line information stored before it, as to not create redundant information.

 Normally when storing OpCode we just store the Opcode followed by its line.

  OPCODE LINE

 In practice the memory layout could look something like this

  OPCODE LINE OPCODE LINE CONSTANT LINE VALUE OPCODE CONSTANT LINE VALUE

  */


#[derive(Debug)]
pub struct Chunk {
    data: *const u8,
    size: usize,
    capacity: usize
}

impl Chunk {
    pub fn new(capacity: usize) -> Self {
        unsafe {
            Self { data: alloc(Layout::array::<u8>(capacity).unwrap()), size: 0, capacity }
        }
    }


    pub fn add_opcode(&mut self, instruction: OpCode, line: u32) {
        while self.size + INSTRUCTION_SIZE >= self.capacity {
            self.resize();
        }

        unsafe {
            write(self.data.add(self.size) as *mut OpCode, instruction);
            write(self.data.add(self.size + OPCODE_SIZE) as *mut u32, line);
        }

        self.size += INSTRUCTION_SIZE;
    }

    pub fn add_value(&mut self, value: Value, line: u32) {
        self.add_opcode(OpCode::Constant, line);

        while self.size + VALUE_SIZE > self.capacity {
            self.resize();
        }

        unsafe {
            write(self.data.add(self.size) as *mut Value, value);
        }

        self.size += VALUE_SIZE;
    }

    pub fn read_opcode(&self, index: usize) -> Option<(OpCode, u32)> {
        if index + INSTRUCTION_SIZE > self.size {
            return None;
        }

        unsafe {
            let opcde: OpCode = read(self.data.add(index) as *const OpCode);
            let line: u32 = read(self.data.add(index + OPCODE_SIZE) as *const u32);

            Some((opcde, line))
        }
    }

    pub fn read_value(&self, index: usize) -> Option<(Value, u32)> {
        if index + VALUE_SIZE >= self.size {
            return None;
        }

        unsafe {
            let value: Value = read(self.data.add(index) as *const Value);
            let line: u32 = read(self.data.add(index - LINE_SIZE) as *const u32);

            Some((value, line))
        }
    }

    fn resize(&mut self) {
        let new_capacity: usize = self.capacity * 2;

        let old_layout: Layout = Layout::array::<u8>(self.capacity).unwrap();
        let new_layout: Layout = Layout::array::<u8>(new_capacity).unwrap();

        unsafe {
            let temp: *mut u8 = alloc(new_layout);
            copy_nonoverlapping(self.data, temp, self.size);
            dealloc(self.data.cast_mut(), old_layout);
            self.data = temp;
        }

        self.capacity = new_capacity;
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        let old_layout: Layout = Layout::array::<u8>(self.capacity).unwrap();

        unsafe {
            dealloc(self.data.cast_mut(), old_layout);
        }
    }
}

pub fn print_chunk(chunk: &Chunk, name: &str){
    println!("=== {name} ===");

    let mut index: usize = 0;

    while let Some(buffer) = chunk.read_opcode(index) {
        match buffer.0 {
            Return => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Negate => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Equal => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Greater => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Less => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Add => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Subtract => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Multiply => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Divide => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Print => {println!("{index:06} {:?}", buffer.0); index += INSTRUCTION_SIZE},
            Constant => {
                print!("{index:06} {:?}", buffer.0);
                index += INSTRUCTION_SIZE;
                let value: &(Value, u32) = &chunk.read_value(index).unwrap();
                match &value.0 {
                    Value::Object(object) => println!(" | {}", object),
                    _ => println!(" | {:?}", value.0)
                }
                index += VALUE_SIZE;
            }
        }
    }
}
