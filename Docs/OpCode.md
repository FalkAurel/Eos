Opcode is an enum. This enum encodes the operations for the [[VM]].  Each OpCode is 1 Byte in size. 

## Implementation Details

```rust
pub enum OpCode{
    Return,
    Constant,
    Negate,
    Print,
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide
}
```


| Enum Fields | Definition                                                                            |
| ----------- | ------------------------------------------------------------------------------------- |
| `Return`    | **Undefined**                                                                         |
| `Constant`  | Instruction to tell the VM that the next Value should be interpreted as a [[Value]]   |
| `Negate`    | Used to invert certain Values                                                         |
| `Print`     | Instruction to print the last element on the stack                                    |
| ```Equal``` | Instruction to compare two [[Value]] for equality. For more see [[Binary Operation]]. |
| `Greater`   | Instruction to compare two [[Value]] for ordering. For more see [[Binary Operation]]. |
| `Less`      | Instruction to compare two [[Value]] for ordering. For more see [[Binary Operation]]. |
| `Add`       | Instruction to add two [[Value]]. For more see [[Binary Operation]].                  |
| `Subtract`  | Instruction to subtract two [[Value]]. For more see [[Binary Operation]].             |
| `Multiply`  | Instruction to multiply two [[Value]]. For more see [[Binary Operation]].             |
| `Divide`    | Instruction to divide two [[Value]]. For more see [[Binary Operation]].               |
