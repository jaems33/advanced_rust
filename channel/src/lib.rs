use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
  shared: Arc<Shared<T>>,
}

// Still don't fully understand why this needs to be re-implemented, should re-watch 36:13
impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    let mut inner = self.shared.inner.lock().unwrap();
    inner.senders += 1;
    drop(inner);
    Sender {
      // Clone the arc, not the thing inside the arc
      shared: Arc::clone(&self.shared),
    }
  }
}

impl<T> Drop for Sender<T> {
  fn drop(&mut self){
    let mut inner = self.shared.inner.lock().unwrap();
    inner.senders -= 1;
    let was_last = inner.senders == 0;
    drop(inner);
    if was_last {
      self.shared.available.notify_one();
    }
  }
}

impl <T> Sender<T> {
  pub fn send(&mut self, t: T) {
    let mut inner = self.shared.inner.lock().unwrap(); // unwrap returns the Some
    inner.queue.push_back(t);
    // Drop the lock so that whoever we notify can wake up and notify one thread
    // and because we are the sender, we know that available will be the receiver
    drop(inner);
    self.shared.available.notify_one();
  }
}

pub struct Receiver<T> {
  shared: Arc<Shared<T>>,
}

impl <T> Receiver<T> {
  pub fn receive(&mut self) -> Option<T> {
    let mut inner = self.shared.inner.lock().unwrap();
    loop {
      match inner.queue.pop_front() {
        Some(t) => return Some(t),
        None if inner.senders == 0 => return None,
        None => {
          inner = self.shared.available.wait(inner).unwrap(); // Wait puts us to sleep until there's a reason to wake up
        }
      }
    }
  }
}

struct Inner<T> {
  queue: VecDeque<T>,
  senders: usize,
}

struct Shared<T> {
  inner: Mutex<Inner<T>>,
  available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>){
  let inner = Inner {
    queue: VecDeque::<T>::default(),
    senders: 1
  };
  let shared = Shared {
    inner: Mutex::new(inner),
    available: Condvar::new(),
  };
  let shared = Arc::new(shared);
  (
    Sender {
      shared: shared.clone(),
    },
    Receiver {
      shared: shared.clone(),
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
    assert_eq!(rx.receive(), Some(42));
  }

  #[test]
  fn closed(){
    let (tx, mut rx) = channel::<()>();
    drop(tx);
    assert_eq!(rx.receive(), None);
  }
}