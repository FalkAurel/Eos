use super::chunk::{Chunk, VALUE_SIZE, INSTRUCTION_SIZE};
use super::common::{DEFAULT_STACK_CAPACITY, runtime_error};
use super::opcode::OpCode::*;
use super::value::{Negate, Value, Comparison};

#[derive(Debug)]
pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    line: u32,
    ptr: usize // it serves as an index into the arraylist bytes
}

impl <'a> VM <'a>{
    pub fn new(chunk: &'a Chunk) -> Self {
        Self { chunk, stack: Vec::with_capacity(DEFAULT_STACK_CAPACITY), line: 0, ptr: 0 }
    }

    pub fn run(&mut self) -> Option<()>{
        while let Some(buffer) = self.chunk.read_opcode(self.ptr) {
            self.line = buffer.1;

            match buffer.0 {
                Equal => self.equal(),
                Greater => if let Err(err) = self.binary_op(|a, b| a.greater(&b) ) {
                    self.error(&err);
                    return None;
                },
                Less => if let Err(err) = self.binary_op(|a, b| a.less(&b) ) {
                    self.error(&err);
                    return None;
                },
                Add => if let Err(err) = self.binary_op(|a, b| a + b) {
                    self.error(&err);
                    return None;
                },
                Subtract => if let Err(err) = self.binary_op(|a, b| a - b) {
                    self.error(&err);
                    return None;
                },
                Multiply => if let Err(err) = self.binary_op(|a, b| a * b) {
                    self.error(&err);
                    return None;
                },
                Divide => if let Err(err) = self.binary_op(|a, b| a / b) {
                    self.error(&err);
                    return None;
                },
                Negate => if let Err(err) = self.negate() {
                    self.error(&err);
                    return None;
                },
                Constant => self.push_to_stack(),
                Print => self.print(),
                Return => self.move_ptr(INSTRUCTION_SIZE)
            }
        }
        Some(())
    }

    pub fn get_stack(&self) -> &[Value] {
        &self.stack
    }

    fn binary_op<F: Fn(Value, Value) -> Result<Value, String>>(&mut self, operand: F) -> Result<(), String>{
        self.move_ptr(INSTRUCTION_SIZE);

        let a: Value = self.stack.pop().ok_or_else(|| "EXPECTED TO  A NONE-EMPTY STACK".to_string())?;
        let b: Value = self.stack.pop().ok_or_else(|| "EXPECTED TO  A NONE-EMPTY STACK".to_string())?;

        let temp: Value = operand(b, a)?;

        self.stack.push(temp);

        Ok(())
    }

    fn equal(&mut self) {
        self.move_ptr(INSTRUCTION_SIZE);

        let a: Value = self.stack.pop().expect("EXPECTED TO  A NONE-EMPTY STACK");
        let b: Value = self.stack.pop().expect("EXPECTED TO  A NONE-EMPTY STACK");

        self.stack.push(Value::Boolean(a == b));
    }

    fn negate(&mut self) -> Result<(), String>{
        self.move_ptr(INSTRUCTION_SIZE);

        let temp: Value = self.stack.pop().expect("EXPECTED TO  A NONE-EMPTY STACK");

        self.stack.push(temp.negate()?);

        Ok(())
    }

    fn push_to_stack(&mut self) {
        self.move_ptr(INSTRUCTION_SIZE);

        let value: Value = self.chunk.read_value(self.ptr).expect("EXPECTED VALUE").0;
        self.stack.push(value);

        self.move_ptr(VALUE_SIZE);

    }

    fn print(&mut self) {
        self.move_ptr(INSTRUCTION_SIZE);

        println!("{}", self.stack.pop().expect("EXPECTED TO  A NONE-EMPTY STACK"));
    }

    fn move_ptr(&mut self, amount: usize){
        self.ptr += amount;
    }

    fn error(&self, msg: &str) {
        //let error: &str =
        runtime_error(&format!("At line {}: {}", self.line, msg));
    }
}
