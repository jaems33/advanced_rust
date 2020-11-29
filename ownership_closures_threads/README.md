# Rust Stream: Ownership, Closures, and Threads - Oh My! (Ryan Levick)

- In a closure, you don't take a concrete type you take a generic type that implements one of the closure traits.
- Threads in Rust are OS threads
- **Green threads** are threads scheduled by the runtime library or virtual machine instead of the OS. They require some runtime that keeps track of these threads, which goes against the Rust philosophy. Any kind of abstraction that has a cost should only cost something if actively use it. Green threads have a cost whether they are used or not because you need to startup a runtime. 
- Tokio: You startup a Tokio runtime and gives the ability to spawn tasks. Use the term task instead of thread to avoid the confusion but you can think of them as green threads. 

### MPSC
- Multi Producer, Single Consumer. Allows us to send messages across threads.
- `std::sync::mpsc::channel` is generic to the type so it needs to be set to some type.
- `channel.recv()` returns a result because if the sender is no longer there and the channel is broken then recv will stop blocking and return an error to let you know the channel was broken up.
- `Fn` is a trait but for some instances like using them with a struct you need to know what size they are to help the program know how to partition space for it. We can encapsulate it within `Box`. `dyn` designates a trait as dynamically dispatched.
- The object being sent over the channel is a Box, a heap allocated pointer which points to a closure, which can be called through that pointer.

- When you define a function generically in Rust, it will create instances of those functions based on the types you passed into it
- This is called monomorphization (creating a function for every concrete type of thing that implements the trait you set)
- What if you want one function and you don't care about which concrete type it is but something that implements are particular trait (i.e. the `std::fmt::Debug` trait). So instead of calling some type `T` that happens to implement `Debug`, we literally take the debug trait
```
fn foo<T: std::fmt::Debug>(item: T) {
  println1("Do Something")
}

// Won't work because Rust can't determine the size of std::fmt::Debug
fn foo(item: std::fmt::Debug) {
  println1("Do Something")
}

// Will work because it's a pointer to some object out there and it will go through a vTable to call functions on it
fn foo(item: &dyn std::fmt::Debug) {
  println1("Do Something")
}

```
- Sync is a trait whereby types for which it is safe to share references between threads.
- Receiver is not thread safe 

- What do you do when you have something that does not allow for multiple things to have access at one time and for us to say "we want to have exclusive access?" Mutex.
- Mutex is only `Send` and `Sync` if the thing it's sending is also a `Send`
- Very common pattern for `Mutex`'s in `Arc` for multiple owners
- Add extra constraint to the closure 


### Copy vs Clone
- `Clone` is a Supertrait of Copy, so everything in Copy must also implement Clone. Clone can do arbitrary procedures to create a copied version of it's T.
- `Copy` are values that can be safely duplicated with `memcpy`. It cannot be re-implemented.  
- Clone explicit way to create a copy of a type that can run arbitrary code. Copy is a way of copying the type that takes a memcpy. Clone is more general than Copy.

### General
- Because Rust cares very deeply about performance, it gives a lot of tools to be productive but adds complication

### Static Lifetimes
- `'static` Static lifetimes are good for the lifetime of the program
- 

### FnMut
- `FnMut` are sub-traits of `Fn`, anything that is an `FnMut` is an `Fn`

### Atomics
- Provide primitive shared-memory communication between threads
- When you do an operation the CPU ensures there's only one thing operating on it
- Like mutexes, but guaranteed by the CPU