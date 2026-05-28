use super::stats::Stats;
use std::sync::{Mutex, PoisonError};

pub struct GlobalStats {
  stats: Mutex<Stats>,
}

#[allow(non_snake_case)]
impl GlobalStats {
  pub const fn new(capacityBytes: usize) -> Self {
    Self {
      stats: Mutex::new(Stats::new(capacityBytes)),
    }
  }

  pub fn lock(&self) -> Result<std::sync::MutexGuard<'_, Stats>, PoisonError<std::sync::MutexGuard<'_, Stats>>> {
    return self.stats.lock();
  }
}
