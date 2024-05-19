A simple virtual machine, which allows you to execute Eos-Bytecode. To put it simply, the `VM` reads [instructions](OpCode) from the [[Chunk]] and executes it, almost like following a long tape. The VM fails as soon as any operation fails.

## Implementation Details


```rust
pub struct VM<'a> {
    chunk: &'a Chunk,
    stack: Vec<Value>,
    line: u32,
    ptr: usize // 
}
```

| Struct Fields | Definition                                                 |
| ------------- | ---------------------------------------------------------- |
| `chunk`       | Immutable reference to [[Chunk]]                           |
| `stack`       | Stack to store [Values](Value)                             |
| `line`        | Keeps track of the current line, useful for error messages |
| `ptr`         | Serves as an index into the `chunk`                        |

`VM` exposes 2 public function:

```rust
1. pub fn new(chunk: &'a Chunk) -> Self
2. pub fn run(&mut self) -> Option<()>
```

As the second function suggests, it return `None` if, for whatever reason, the `VM` encounters a [runtime error](Error), otherwise it returns nothing. 

