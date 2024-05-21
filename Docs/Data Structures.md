Currently we support one data structure: ObjString

## ObjString

```rust
    pub struct ObjString {
        data: String,
        hash: usize
    }
```


ObjString is a wrapper for `String` with a hash. The hash can be used to implement a more efficient comparison between ObjStrings, since it reduces the expensive underlying data comparison to the cases where the [hash comparison is successful](https://en.wikipedia.org/wiki/Hash_collision). Furthermore using the hash allows us to perform [string interning](https://en.wikipedia.org/wiki/String_interning), meaning we only store unique strings, reducing the space complexity of the runtime. 
The trade of, of having reduced space and time complexity during runtime, is an initial creation cost.
Every time a new `ObjString` is created, the hash will be recomputed. 

```rust
pub fn compute_hash(string: &String) -> usize {
    string.as_bytes().iter().fold(14695981039346656037, |mut hash: usize, element: &u8| -> usize {
        hash ^= *element as usize;
        hash.wrapping_mul(1099511628211)
    })
}
```

Looking at this very simple [FNV-1a](https://en.wikipedia.org/wiki/Fowler–Noll–Vo_hash_function) implementation, we can see that it run in *O(n)*. Combined with looking it up in a hash table which is a constant time operation *O(1)*, we have a considerable cost to pay during creation. To make full of use of this data structure you may use it during frequent comparisons.
