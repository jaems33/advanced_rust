use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
  inner: Arc<Inner<T>>,
}

// Still don't fully understand why this needs to be re-implemented, should re-watch 36:13
impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Sender {
      // Clone the arc, not the thing inside the arc
      inner: Arc::clone(&self.inner),
    }
  }
}

impl <T> Sender<T> {
  pub fn send(&mut self, t: T) {
    let mut queue = self.inner.queue.lock().unwrap(); // unwrap returns the Some
    queue.push_back(t);
    // Drop the lock so that whoever we notify can wake up and notify one thread
    // and because we are the sender, we know that available will be the receiver
    drop(queue);
    self.inner.available.notify_one();
  }
}

pub struct Receiver<T> {
  inner: Arc<Inner<T>>,
}

impl <T> Receiver<T> {
  pub fn receive(&mut self) -> T {
    let mut queue = self.inner.queue.lock().unwrap();
    loop {
      match queue.pop_front() {
        Some(t) => return t,
        None => {
          queue = self.inner.available.wait(queue).unwrap(); // Wait puts us to sleep until there's a reason to wake up
        }
      }
    }
  }
}

struct Inner<T> {
  queue: Mutex<VecDeque<T>>,
  available: Condvar,

}

pub fn channel<T>() -> (Sender<T>, Receiver<T>){
  
  let inner = Inner {
    queue: Mutex::default(),
    available: Condvar::new(),
  };
  let inner = Arc::new(inner);
  (
    Sender {
      inner: inner.clone(),
    },
    Receiver {
      inner: inner.clone(),
    },
  )
}

mod tests {

  #[allow(unused)]
  use super::*;

  #[test]
  fn ping_pong(){
    let (mut tx, mut rx) = channel();
    tx.send(42);
    assert_eq!(rx.receive(), 42);
  }
}