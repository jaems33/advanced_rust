# Key Value Store
A simple key-value store that's exported a database. 

Written from from [Introduction to Rust Part 1 from Ryan Levick](https://www.youtube.com/watch?v=WnWGO-tLtLA)

### General Notes
- Rust is a systems language, meant for low-level software (OS, Browsers) like C/C++
- Can be used to build end-user software, high-level
- `expect()` can be used for error messaging when receiving `None`
- `format!` is similar to `println!` but instead returns a string
- `xxd kv.db` allows you to read the file in binary format where values are in hex format
- `Database` returns a `Result` because if something goes wrong, it shouldn't just return a Map
- It's an imperative language like C/C++ but has borrowings from Function programmming
- Does not have a garbage collector looking at items when they aren't used anymore and GCing them away. Instead of calling `free` on an item like in C/C++, in Rust what you say is that each binding within a block has an owner. For example, a `string` is bound to variable `key`. What ownership means is that when the binding goes out of scope from a context, the memory it has ownership over will be dropped.
- Transferring ownership of bindings, or `move`ing elements
- String views are string slices. Can have many slices into the string, but only one owned value.
- Dangling pointers are mostly impossible because you can't have a reference to something after its dropped
- A `clone` of a reference is still a reference

### Result
- If something goes wrong, you typically throw exceptions
- Rust does not have exceptions, it has panics
- Panics kill the program
- For errors where there is some way of recovering, we don't panic we return `Result` types.
- It's a type that shows either sucess or error of some type
- Result of `std::fs::write()` is either () or Error.
- `()` is an empty Unit, similar to Void. A type representing something that's not very interesting.
- Using `.unwrap()` unwraps the variable if it's okay, and crashes if it breaks
- `std::io::Result` is just a specialization on the normal Result type where the error type is standard to `std::io::Error`, because the errors are io errors
- `.expect()` is what causes the error to crash
- It's an enum with two variants, can be generic to two types. Enums can have data associated with it.

### Option
- Denotes it's there or not

### Pattern Matching
- Like a switch statement, but much better
- Match expressions are not just for errors
- It's an expression, not a statement. So you can bind to it. Regular `if` statements can also be bound.

### Struct
- Rust doesn't have constructors
- `private` fields are still viewable within the same file
- Create the struct with its fields and then somewhere else you have its `impl` where you add the methods and associated functions
- Don't have to use a specific name, can use `new()` or `create()` for the method that returns the struct
- Structs are **stack allocated** so the return is not a pointer or a reference, it's living on the stack
- `new` does not imply allocation in Rust

### Hashmap
- `use std::collections::HashMap;` to bring it into scope so that you can use `HashMap`

### Owned
- `to_owned`