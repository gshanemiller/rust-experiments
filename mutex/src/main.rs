use std::thread;
use std::sync::{Mutex, PoisonError};

struct Stats {
  a: usize,
  b: usize,
}

impl Stats {
  const fn new(cap: usize) -> Self {
    Self {
      a: cap,
      b: 0,
    }
  }

  fn av(&self) -> usize {
    return self.a;
  }

  fn bv(&self) -> usize {
    return self.b;
  }

  fn inc(&mut self) {
    self.a += 1;
  }
}

struct GlobalStats {
  data: Mutex<Stats>,
}
  
impl GlobalStats {
  const fn new(cap: usize) -> Self {
    Self {
      data: Mutex::new(Stats::new(cap)),
    }
  }

  fn stats(&self) -> Result<std::sync::MutexGuard<'_, Stats>, PoisonError<std::sync::MutexGuard<'_, Stats>>> {
    return self.data.lock();
  }
} 

fn createThreads<F>(func: F)
where
  F: Fn() + Send + Sync + 'static + Copy,
{
  let mut handles = Vec::with_capacity(10);

  for _ in 0..10 {
    let handle = thread::spawn(move || {
      func();
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().expect("Thread panicked!");
  }
}

static G: GlobalStats = GlobalStats::new(10);

fn main() {
  createThreads(|| {
    let mut s = G.stats().unwrap();
    s.inc();
    println!("thread {:?} {} {}", thread::current().id(), s.av(), s.bv());
  });
}
