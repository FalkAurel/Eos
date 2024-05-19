The Chunk is in itself a custom made [array list](https://en.wikipedia.org/wiki/Dynamic_array). The properties of an array list or dynamically allocated array are very favourable, as it supports constant time operations *O(1)* for reading values by index and appending values to the data structures, the only two operations needed for the chunk. 

## Implementation Details

```rust
pub struct Chunk {
    data: *const u8,
    size: usize,
    capacity: usize
}
```

| Struct Fields | Usage                                                   |
| ------------- | ------------------------------------------------------- |
| `data`        | Immutable pointer to an u8-byte                         |
| ```size```    | Number of currently stored elements, in this case bytes |
| `capacity`    | Max. size before resizing                               |

```Chunk```has a growth factor of 2, meaning every time it resize, the capacity doubles.

Resizing in itself is a dangerous operation. It directly manipulates memory, meaning it can be the source of bugs, and it forces you to have a capacity of at least 1, as  0 * 2 = 0.

```rust
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
```

```Chunk``` exposes five public methods. [[OpCode]] and [[Value]] are types used to represent data.
OpCode encodes instructions, while Value stores values.

```rust
1. pub fn new(capacity: usize) -> Self
2. pub fn add_opcode(&mut self, instruction: OpCode, line: u32)
3. pub fn add_value(&mut self, value: Value, line: u32)
4. pub fn read_opcode(&self, index: usize) -> Option<(OpCode, u32)>
5. pub fn read_value(&self, index: usize) -> Option<(Value, u32)>
```


