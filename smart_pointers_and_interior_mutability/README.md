# Notes from Crust of Rust: Smart Pointers and Interior Mutability (Jon Gjengset)
## Cell
- Shareable, mutable containers
- Shared reference to something, yet you're still allowed to mutate it? Mutable references exists but have a major restriction: in a particular scope only one mutable reference to a specific object can exist at one time.
- This module is various container types that allow you to do this in a controlled fashion
- This is an example of interior mutability. Externally it looks immutable, but interally there's methods to mutate it
- `RefCell`, `Mutex`, and `Cell` have different restrictions as to what you can stick inside of them and how you can use them. Generally, the farther you go towards `Mutex` the freer you are to put whatever you want inside. But the overhead of required logistics also increase. A type does not know whether or a not another type has interior mutability. 
- `RefCell`
- `Mutex`: Not in `Cell` but in `Sync` because it uses synchronization primitives provided by the OS or CPU to make the operations safe. You can think of Mutex as a type of cell, a type of interior mutability.
- `Cell`: Can store any type in a cell. 
- `set(&self, val: T)` has an immutability reference, but still allows you to modify the value contained within the cell
- Cell also has a swap method which allows you to take references to two cells and swap the values inside of them
- `into_inner(self)` consumes itself, gives copy.
- `get(&self)` does not return a reference to what's inside the `Cell` but instead copies what inside and gives you a new copy of the value.
- None of the methods of `Cell` give a reference to what's inside of it. You can replace it, change it, and get a copy of it, but not get a pointer.
- If there's no way for you to get a reference to a cell, then it's always safe to mutate it. Because if no one else has a pointer to it, then changing it is fine.
- Cell does not implement `sync`. If you have a reference to Cell, you can't give away that reference to a different thread. If I have two threads that both have a mutable reference to the cell, then both threads could try to change the value of the Cell at the same time.
- The benefit `Cell` provides is the ability to have multiple shared references to a thing. Usually `Cell` is used with `rc` where you want the cell to be stored in multiple places. Or pointers to be stored in multiple places. Because it's single threaded, you know you'll only be using one of the references at a time. What cell does is lets you in safe code mutate that value.
- Should generally be used only for small data types, since copying is the only way you can get their values. An example would be flags that can be mutated from multiple different places. Often used with thread locals (aka static or local memory to a thread, like a flag or a counter).


## UnsafeCell
- Totally unsafe to use
- Holds some type, you can get a raw, exclusive pointer to it anytime you want, but it's up to you to cast that into an exclusive Rust reference when you know its safe to do so
- 

## Unsafe
- Writing unsafe means "I have checked that no one else is currently mutating this value, 

### Reminders:
- `&T` is a shared reference
- `&mut T` is an exclusive reference. No other reference to same value can exist at the same time. In contrast, a shared reference means other references to the same value might exist, including in other threads (if `T` implements `Sync`)