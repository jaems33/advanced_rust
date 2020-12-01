# Linked List
## From [A Singly Linked List in Rust by Ryan Levick](https://www.youtube.com/watch?v=IiDHTIsmUi4)
- 
- In C++, a node you'd normally have two properties of a simple linked list: A value, and a pointer to another Node that either exists or is null
- Rust doesn't have null
- Everything in Rust is Stack allocated, so when you create a Node Rust needs to know how much space it's going to take. So the tricky part in having `NonEmpty(u32, Node)` as an enum variant is that you could have any number of NonEmpty's
- Must add some indirection to make `Node` representable, such as a `Box`
- When you rip out the old list, you don't have anything else sitting in memory 
- Need a way to take the head of the list and replacing it with something else and give access to the previous head of the list

## Lifetimes
```
enum Node {
  Empty,
  NonEmpty(u32, &Node)
}

```
- This code results in an error as Rust will ask for a lifetime specifier
- You have a reference/pointer to a Node that's going to live for a certain amount of time and we need to know how long does it live for.
- We don't need this for `Box` because it is an owned pointer. We decide when we have a value it's point to will get destroyed.
- In comparison with reference we don't decide that, we're simply borrowing the value. Some other variable in the code that owns the value and will decide to destroy it. Since we're borrowing it, we have to know how long it lives for because we have to prove to the compiler that it lives long enough.

```
enum Node<'a> {
  Empty,
  NonEmpty(u32, &'a Node<'a>)
}
```
- So you might add a lifetime, Node is generic on lifetime **a**.
- When you have a Node, it is constrained to live as long as lifetime a, however long the reference contained inside of it lives for
- `std::mem::replace(dest, src)` takes a mutable reference to something and an owned value, and places the owned value to where the mutable reference is pointing to and returns back the old value.
- `&mut` provides exclusive access
- The invalid state where head is temporarily set to `None` is potentially a business logic error where the list has been made empty
- Having the head be invalid memory opens up to potential nasty bugs that could be dangerous for security
- Rust is a memory safe language where we always want to have memory safe view of the world
- You cannot double free memory in Rust

## Box
- A `Box` is a heap-allocated pointer, akin to a `unique_ptr` Pointer
- Owns it's value it points to
- Things are dropped when `Box` goes out of scope

## Todo Macro
- `todo!("Message")` just let the code compile

