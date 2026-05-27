use super::stats::Stats;
use std::sync::{Mutex, PoisonError};

pub struct GlobalStats {
  stats: Mutex<Stats>,
}

impl GlobalStats {
  pub const fn new(cap: usize) -> Self {
    Self {
      stats: Mutex::new(Stats::new(cap)),
    }
  }

  pub fn lock(&self) -> Result<std::sync::MutexGuard<'_, Stats>, PoisonError<std::sync::MutexGuard<'_, Stats>>> {
    return self.stats.lock();
  }
}
