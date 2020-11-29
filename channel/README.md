# Channels
## Via Jon Gjengset's Crust of Rust: Channels 
- Channels are a way to send information from one place to another
- `std::sync::mpsc` - Multi Producer Single Consumer -> Many senders, one receiver
- Receiver type, Sender type, SyncSender type
- Channels are uni-directional, only senders can send and only receivers can receive
- You can clone the sender but not the receiver
- Sender and Receiver are parameterized by the type of the thing they are going to send and receive
- You can construct a channel and send stuff that isn't Send as long as the sender and receiver are not moved across the thread boundary
- Can send any type on the channel, it's not serialization, not TCP
- T has to be sized, and the Channel owns the T
- Example: One thread is an Event loop and it might send to itself 
- **Backpressure** is the resistance or flow opposing flow of data through software. Reading is faster than writing, so if you have to do both at the same time, you can't pay off that debt until after you finished reading. It can occur when one server is sending requests faster than one server can process them. Another example is sending information to the DOM is cheaper than actually rendering the information. So on the client side, information needs to be buffer or dropped. Three main strategies: control the producer, buffer by accumulating data spikes, or drop some of the data. Source: https://medium.com/@jayphelps/backpressure-explained-the-flow-of-data-through-software-2350b3e77ce7
- **Mutex**: Mutual exclusion, you have a `lock` method, which returns a `guard`, and while you have that guard you are guaranteed to be the only thing that can access the T protected in the mutex. If two threads try to lock the same mutex, one will get to go and the other will block (aka wait) til the other released the `guard` at which point it can go. This ensures only one thread modifying the T at any point in time.
- Mutex returns **LockResult**. Imagine the last thread to take the lock panicked while holding the lock. That might mean the data under the lock is in an inconsistent state. The way the lock communicates this is when the thread panics it releases the lock but sets a flag in it to say the last thing that accessed this panicked. Thus, a LockResult is either a `Guard` or a `PoisonError<Guard>`. 
- **Arc**: A reference count type (Atomically Reference Counted type), which means it can be used across thread boundaries. Arc is needed because otherwise the Sender and Receiver would have two different instances of Inner. And if they did, how would they communicate? They need to share an inner because that's where the sender is going to put data and where the receiver is going to take data out of.
- **Condvar**: Conditional variable is a way to announce to a different thread you've changed something it cares about. E.g. a receiver is sleeping till it receives something, the sender has to wake up the thread that's sleeping to inform it that there's information for it to read. 
- A mutex is like a boolean semaphore (a boolean flag you check and atomically update). If the flag is set and something else is in the critical section and has the mutex, with a boolean semaphore you have to spin and repeatedly check to see it. With a mutex, the OS can put the thread to sleep and wake it back up when the mutex becomes available.
- The problem of course with using a `Vector` as a queue is that removing elements from the beginning results in having to shift all the elements over to fill the hole that was removed, so a better alternative is a `RingBuffer`. Rust also has `VecDeque` which is sort of like a vector but stores start and end position seperately. So if you pop from the beginning, it just moves the pointer to where the data starts. The data might end up wrapping around but it can be used as a queue rather than a stack.
- Blocking version of Receive: Provide a receive whereby if there isn't something yet, it waits for something to be in the channel.
- `Condvar` needs to be outside the `mutex`, because a thread is holding the mutex and you need to wake another thread up. The person you wake up has to take the mutex but because you're holding the mutex they'll go back to sleep because you're still holding it. After you finish, no thread is awake causing a **deadlock.** `Condvar` is let go at the same time you notify the other thread. Thus, `Condvar` requires an input of a mutex guard, so that you prove you own the lock and it'll make sure it does the step as an `atomic` step.
- `atomic` actions that happen all at once. It either happens completely or nothing happens at all. No side effects are noticeable until the action is complete. No other process can interfere with the manipulation of data by an atomic action. 
- `wait` gives a mutex guard back. If you get woken up, you automatically get handed the mutex. Thus, to use wait, you must pass in a guard. On a `condvar`  
- Async/Await is generally when you are I/O bound not CPU bound
- `notify_one()` does not guarantee which thread will be woken up
- `Arc::strong_count(self)` gives how many references there are to that arc. 
- Generally shouldn't wake up threads unless you need to (related to performance rather than correctness)
- Difference between synchronous and asynchronous channels: Whether sends can block. In a sychronous channel, the senders and receivers are synchronized. Imagine a sender that's much faster than the receiver. In a synchronous channel, the channel has a capacity and if the channel is filled than the sender blocks. 
- The advantage of a synchronous channel is that there is backpressure, the sender will eventually start blocking as well. 
- If the sender is blocking but the receiver receives something, the receiver can tell the sender that it's okay to send
- `sync_channel`: takes a `usize` bound which is the channel's capacity, returns a `SyncSender` and `Receiver`. 
- `weak` is a version of `Arc` that doesn't increment the reference count, but you have a way to try to update the reference count if the reference count hasn't already gotten down to zero. One downside of `weak` is everytime you try to send you have to atomically update the reference count and decrement it after which adds overhead.

- Calling push_back on a `vector` is not necessarily free as the data structure might to be allocated elsewhere (while the previous instance is deallocated). Resizing isn't blocking, the resize takes longer and in the meantime you can't do sends or receives. In practice, you don't use a deque. 

## Optimization of receiver 1:07:38 
- Because there's only one receiver, any time you take the lock you should take all the items that have been queued up rather than just the one because no one else is going to take them. If you call receive again, you can just call a local buffer of the things that were taken last time. So whenever someone calls receive, check the last time we took the lock and if there were leftover items at the time, and if so return from there you don't need to take the lock. 
- If the buffer is empty, then take the lock. If you do take the lock, try to take the front item. If the queue is empty do the same thing as before and wait. But if it's not empty, then check if there are more items in the queue, and if there are take ownership of all of them. Swap that vec deque, with the one we've buffered inside of ourselves
- Thus instead of taking the lock on every receive, we only take the lock once every time there were no additional sends between locks.
- It is true that it may double the memory because you have two vec deques that are both growing as you add more items, and you'll be swapping between them
- The lock is taken fewer times, so the lock will be faster to acquire

- `Branch predictor`: the CPU has a built-in component that observes conditional jumps, and it tries to remember whether it took the branch or not the branch last time it was here, whereby speculative execution comes into play. If it runs the code again, the branch predictor will whether or not it'll take the branch so start running the code under the assumption that it will or won't. If it doesn't end up doing that, then go back and unwind what was done and do the other stuff instead.

## Channel Flavors
- The idea behind flavors is you have multiple implementations of channels and you choose the channel based on how it's used
- Synchronous Channels: Channel where `send()` can block, limited capacity. Sometimes called Bounded channels. Implemented with mutex + condvar. Use a vecdeque, have the sender block if the vecdeque is full. If you don't want to use a mutex, use an atomic queue/vecdeque, with head and tail pointers. And update the pointers atomically.
- Asynchronous Channels: Channel where `send()` cannot block, usually unbounded capacity (any number of sends). Sometimes called Unbounded channels. Mutex + Condvar + LinkedList, so that you never resize and just push to the front of the LinkList. The receiver steals the entire linkedlist, setting the head to none, and walks it backwards (so sometimes implemented as a Doubly LinkedList). Doesn't have the memory problems that vecdeque does. For a non-mutex solution, use an `AtomicLinkedList` or `AtomicQueue`. 
- Rendezvous Channels: Synchronous channel with capacity = 0. Doesn't let you send things, usually used to synchronize two sides, thread syncing. What capacity zero means is you can only send if there's a currently blocking receiver because you can't store anything in the channel itself. So the only way to send is to hand it over to a thread that's currently waiting. It is not a mutex, because it does not guarantee mutual exclusion. Sort of like a convar in that you can wake up a thread. Kind of like a baton pass.
- Oneshot Channels: Channels that you only send on once. Any capacity, in practice only one call to `send()`. E.g. an application with a channel used to tell all threads to exit early like when a user ctrl-c's a program. 


## Async / Await
- If you do a send and the channel is full, in the async/await world, you don't want to block you want to yield to the parent executor/task, and at some point in the future you'll be woken up to pull again. Sounds like waiting on a Condvar, but not the same because you need to return instead of sitting in the current function. 
- Flume and Crossbeam have both blocking and async versions, requiring more book-keeping