Value is a [tagged union](https://en.wikipedia.org/wiki/Tagged_union). Tagged unions are data structures, whose size matches the biggest element. This property allows for efficient memory usage, picture this: You can store in 2 bytes one byte.  Doing this also allows us to have always a fixed size data type whilst still retaining the properties of being dynamically typed.

## Implementation Details


```rust
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(DynType),
    Null,
}
```

```Value``` contains two kinds of data types: primitive datatypes and objects.

### Primitive Data

Primitive Datatypes are types which can be trivially copied, meaning they don't store any other information besides what they represent. Therefore `Integers`, `Floats` and `Booleans` are directly represented as primitive data types. 

### Objects

Objects are dynamically allocated types. Eos allocates Objects on the heap, meaning only a pointer to the heap location is stored in `Value`. 
Due to type constraints imposed by the [[VM]] we are forced to use non-generic types. Luckily we can leverage the [enum-dispatch trick](https://docs.rs/enum_dispatch/latest/enum_dispatch/) to retain much of the speed of static dispatch, although we're still forced to follow one fat pointer, whilst preserving the benefits of dynamic dispatch on trait objects. The performance gain can be as great as 10x compared to using trait objects.


```rust
pub enum DynType {
    Text(Box<String>),
    HashMap(Box<HashMap<Value, Value>>)
}
```


To learn more about `DynType` follow [[Data Structures]].

### Traits

Eos uses under the hood traits to define type specific behaviour. This design choice creates a clean and extendable interface. Following this design pattern allows for greater flexibility through abstraction: you don't need to know how `+` is implemented for Strings or Matrices, thereby also opening a new opportunity for programmers to define their own data types and integrate them seamless into the language. 

| Trait        | Definition                                    |
| ------------ | --------------------------------------------- |
| `Add`        | Allows you to interface with the `+` operator |
| `Sub`        | Allows you to interface with the `-` operator |
| `Mul`        | Allows you to interface with the `*` operator |
| `Div`        | Allows you to interface with the `/` operator |
| `Comparison` | Allows you to interface with `>` and `<`      |
| `PartialEq`  | Allows you to interface with `==`             |
| `Negate`     | Allows you to interface with `!` and `-`      |
| `Display`    | Allows you to print data to the console       |
