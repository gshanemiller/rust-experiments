#[cfg(all(feature="debugAllocatorStats"))]
use std::ffi::CStr;
#[cfg(all(feature="debugAllocatorStats"))]
use std::alloc::{Layout};
#[cfg(all(feature="debugAllocatorStats"))]
use std::os::raw::c_char;

#[cfg(all(feature="debugAllocatorStats"))]
unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

#[cfg(all(feature="debugAllocatorStats"))]
#[derive(Clone, Copy)]
#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Stats {
  d_capacityBytes:        usize,
  d_allocatedBytes:       usize,
  d_maxAllocatedBytes:    usize,
  d_freeCount:            usize,
  d_allocCount:           usize,
  d_totalFreedBytes:      usize,
  d_totalAllocatedBytes:  usize,
}

#[cfg(all(feature="debugAllocatorStats"))]
#[allow(non_snake_case)]
impl Stats {
  pub const fn new(capacity: usize) -> Self {
    debug_assert!(capacity>0);
    Self {
      d_capacityBytes: capacity,
      d_allocatedBytes: 0,
      d_maxAllocatedBytes: 0,
      d_freeCount: 0,
      d_allocCount: 0,
      d_totalFreedBytes: 0,
      d_totalAllocatedBytes: 0,
    }
  }

  pub fn capacityBytes(&self) -> usize {
    return self.d_capacityBytes;
  }

  pub fn allocatedBytes(&self) -> usize {
    return self.d_allocatedBytes;
  }

  pub fn maxAllocatedBytes(&self) -> usize {
    return self.d_maxAllocatedBytes;
  }

  pub fn freeCount(&self) -> usize {
    return self.d_freeCount;
  }

  pub fn allocCount(&self) -> usize {
    return self.d_allocCount;
  }

  pub fn totalFreedBytes(&self) -> usize {
    return self.d_totalFreedBytes;
  }

  pub fn totalAllocatedBytes(&self) -> usize {
    return self.d_totalAllocatedBytes;
  }

  pub fn freeBytes(&self) -> usize {
    return self.d_capacityBytes-self.d_allocatedBytes;
  }

  pub fn countAlloc(&mut self, layout: Layout) {
    debug_assert!(layout.size()>0);
    self.alloc(layout.size());
  }

  pub fn countDealloc(&mut self, layout: Layout) {
    debug_assert!(layout.size()>0);
    debug_assert!(layout.size()<=self.d_allocatedBytes);
    self.free(layout.size());
  }

  pub fn dump(&self) {
    let cstr1: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: Default Allocator Stats\n\0").unwrap();
    let cstr2: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: -------------------------------------\n\0").unwrap();
    let cstr3: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: FreeBytes:            %lu\n\0").unwrap();
    let cstr4: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: AllocatedBytes        %lu\n\0").unwrap();
    let cstr5: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: MaxAllocatedBytes     %lu\n\0").unwrap();
    let cstr6: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: AllocCount            %lu\n\0").unwrap();
    let cstr7: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: FreeCount             %lu\n\0").unwrap();
    let cstr8: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: CapacityBytes         %lu\n\0").unwrap();
    let cstr9: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: TotalFreedBytes       %lu\n\0").unwrap();
    let cstrA: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: TotalAllocatedBytes   %lu\n\0").unwrap();
    let cstrB: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: PercentAllocated      %-5.2lf\n\0").unwrap();
    let percent = (self.allocatedBytes() as f64)/(self.capacityBytes() as f64) * 100.0;

    unsafe {
      printf(cstr1.as_ptr());
      printf(cstr2.as_ptr());
      printf(cstr3.as_ptr(), self.freeBytes());
      printf(cstr4.as_ptr(), self.allocatedBytes());
      printf(cstr5.as_ptr(), self.maxAllocatedBytes());
      printf(cstrB.as_ptr(), percent);
      printf(cstr2.as_ptr());
      printf(cstr6.as_ptr(), self.allocCount());
      printf(cstr7.as_ptr(), self.freeCount());
      printf(cstr2.as_ptr());
      printf(cstr8.as_ptr(), self.capacityBytes());
      printf(cstr9.as_ptr(), self.totalFreedBytes());
      printf(cstrA.as_ptr(), self.totalAllocatedBytes());
    }
  }

  fn alloc(&mut self, bytes: usize) {
    debug_assert!(bytes>0);
    self.d_allocCount += 1;
    self.d_allocatedBytes += bytes;
    self.d_totalAllocatedBytes += bytes;
    if self.d_allocatedBytes>self.d_maxAllocatedBytes {
      self.d_maxAllocatedBytes = self.d_allocatedBytes;
    }
  }

  fn free(&mut self, bytes: usize) {
    debug_assert!(bytes>0);
    debug_assert!(bytes<=self.d_allocatedBytes);
    self.d_freeCount += 1;
    self.d_allocatedBytes -= bytes;
    self.d_totalFreedBytes += bytes;
  }
}
