/*
  Code written up via Rust Stream: Ownership, Closures, and Threads - Oh My! Tutorial from Ryan Levick
*/

use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::sync::Arc;

pub struct ThreadPool {
  /* Join handle returns nothing to us */
  _handles: Vec<std::thread::JoinHandle<()>>,
  sender: Sender<Box<dyn FnMut()+Send>>,
}

impl ThreadPool {
  /*
    Closure traits are not ordinary traits but are special exceptions
    <Fn()> means it's a normal closure that takes no arguments.
    <Fn(u8)> -> u32 means it takes a u8 as an arg returns u32 
  */
  pub fn new(num_threads: u8) -> Self {
    let (sender, receiver) = channel::<Box<dyn FnMut() + Send>>();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut _handles = vec![];
      for _ in 0..num_threads {
        let clone = receiver.clone();
        let handle = std::thread::spawn(move || loop {
          let mut work = match clone.lock().unwrap().recv() {
            Ok(work) => work,
            Err(_) => break,
          };
          println!("Start");
          work();
          println!("Finish");
        });
        _handles.push(handle);
      }
    Self {_handles, sender}
  }
  /*
    We have generic type T that implements Fn closure trait
    Takes no arguments, and returns nothing from it, passed in as work
    
  */
  pub fn execute<T: FnMut() + Send + 'static>(&self, work: T) {
    self.sender.send(Box::new(work)).unwrap();
  }
}

mod tests {

  #[allow(unused)]
  use super::*;
  #[test]
  fn it_works() {
    use std::sync::atomic::{AtomicU32, Ordering};
    /*
      Need to include & with self as only including self would mean
      execute takes ownership of self.
    */
    let pool: ThreadPool = ThreadPool::new(10);
    /*
      Parallel piped is a closure with zero arguments
    */
    let nref = Arc::new( AtomicU32::new(0));
    let cloned = nref.clone();
    let some_work = move || { cloned.fetch_add(1, Ordering::SeqCst); };
    pool.execute(some_work.clone());
    pool.execute(some_work);
    std::thread::sleep(std::time::Duration::from_secs(1));
    assert_eq!(nref.load(Ordering::SeqCst), 2);
  }
}
