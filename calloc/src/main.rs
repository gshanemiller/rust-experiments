use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::Mutex;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::process;

unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

extern "C" fn process_cleanup_handler() {
  if let Ok(mut guard) = GLOBAL_STATS.lock() {
    *guard = None;
  }
}

#[allow(non_snake_case)]
#[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
struct AllocStats {
  d_capacityBytes:        usize,
  d_allocatedBytes:       usize,
  d_maxAllocatedBytes:    usize,
  d_freeCount:            usize,
  d_allocCount:           usize,
  d_totalFreedBytes:      usize,
  d_totalAllocatedBytes:  usize,
}

#[allow(non_snake_case)]
#[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
impl AllocStats {
  pub const fn new(capacity: usize) -> Self {
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

  pub fn capacityBytes(&self) -> usize { return self.d_capacityBytes; }
  pub fn allocatedBytes(&self) -> usize { return self.d_allocatedBytes; }
  pub fn maxAllocatedBytes(&self) -> usize { return self.d_maxAllocatedBytes; }
  pub fn freeCount(&self) -> usize { return self.d_freeCount; }
  pub fn allocCount(&self) -> usize { return self.d_allocCount; }
  pub fn totalFreedBytes(&self) -> usize { return self.d_totalFreedBytes; }
  pub fn totalAllocatedBytes(&self) -> usize { return self.d_totalAllocatedBytes; }
  pub fn freeBytes(&self) -> usize { return self.d_capacityBytes-self.d_allocatedBytes; }

  pub fn alloc(&mut self, bytes: usize) {
    let cstr: &CStr = CStr::from_bytes_with_nul(b"alloc %lu bytes\n\0").unwrap();
    unsafe {
      printf(cstr.as_ptr(), bytes);
    }
    self.d_allocCount += 1;
    self.d_allocatedBytes += bytes;
    self.d_totalAllocatedBytes += bytes;
    if self.d_allocatedBytes>self.d_maxAllocatedBytes {
      self.d_maxAllocatedBytes = self.d_allocatedBytes;
    }
    self.dump();
  }

  pub fn free(&mut self, bytes: usize) {
    let cstr: &CStr = CStr::from_bytes_with_nul(b"free %lu bytes\n\0").unwrap();
    let cstr1: &CStr = CStr::from_bytes_with_nul(b"warn: invalid free %lu bytes ignored\n\0").unwrap();
    unsafe {
      printf(cstr.as_ptr(), bytes);
    }
    if bytes<=self.d_allocatedBytes {
      self.d_freeCount += 1;
      self.d_allocatedBytes -= bytes;
      self.d_totalFreedBytes += bytes;
      self.dump();
    } else {
      unsafe {
        printf(cstr1.as_ptr(), bytes);
      }
    }
  }

  pub fn allocZeroed(&mut self, bytes: usize) {
    let cstr: &CStr = CStr::from_bytes_with_nul(b"allocZeroed %lu bytes\n\0").unwrap();
    unsafe {
      printf(cstr.as_ptr(), bytes);
    }
    self.d_allocCount += 1;
    self.d_allocatedBytes += bytes;
    self.d_totalAllocatedBytes += bytes;
    if self.d_allocatedBytes>self.d_maxAllocatedBytes {
      self.d_maxAllocatedBytes = self.d_allocatedBytes;
    }
    self.dump();
  }

  pub fn realloc(&mut self, oldSize: usize, newSize: usize) {
    let cstr: &CStr = CStr::from_bytes_with_nul(b"resize %lu to %lu bytes\n\0").unwrap();
    unsafe {
      printf(cstr.as_ptr(), oldSize, newSize);
    }
    if newSize>oldSize {
      self.d_allocatedBytes += newSize-oldSize;
      self.d_totalAllocatedBytes += newSize-oldSize;
      if self.d_allocatedBytes>self.d_maxAllocatedBytes {
        self.d_maxAllocatedBytes = self.d_allocatedBytes;
      }
    }
    self.dump();
  }

  pub fn dump(&mut self) {
    let cstr1: &CStr = CStr::from_bytes_with_nul(b"Default Allocator Stats\n\0").unwrap();
    let cstr2: &CStr = CStr::from_bytes_with_nul(b"-------------------------------------\n\0").unwrap();
    let cstr3: &CStr = CStr::from_bytes_with_nul(b"FreeBytes:            %lu\n\0").unwrap();
    let cstr4: &CStr = CStr::from_bytes_with_nul(b"AllocatedBytes        %lu\n\0").unwrap();
    let cstr5: &CStr = CStr::from_bytes_with_nul(b"MaxAllocatedBytes     %lu\n\0").unwrap();
    let cstr6: &CStr = CStr::from_bytes_with_nul(b"AllocCount            %lu\n\0").unwrap();
    let cstr7: &CStr = CStr::from_bytes_with_nul(b"FreeCount             %lu\n\0").unwrap();
    let cstr8: &CStr = CStr::from_bytes_with_nul(b"CapacityBytes         %lu\n\0").unwrap();
    let cstr9: &CStr = CStr::from_bytes_with_nul(b"TotalFreedBytes       %lu\n\0").unwrap();
    let cstrA: &CStr = CStr::from_bytes_with_nul(b"TotalAllocatedBytes   %lu\n\n\n\0").unwrap();

    unsafe {
      printf(cstr1.as_ptr());
      printf(cstr2.as_ptr());
      printf(cstr3.as_ptr(), self.freeBytes());
      printf(cstr4.as_ptr(), self.allocatedBytes());
      printf(cstr5.as_ptr(), self.maxAllocatedBytes());
      printf(cstr2.as_ptr());
      printf(cstr6.as_ptr(), self.allocCount());
      printf(cstr7.as_ptr(), self.freeCount());
      printf(cstr2.as_ptr());
      printf(cstr8.as_ptr(), self.capacityBytes());
      printf(cstr9.as_ptr(), self.totalFreedBytes());
      printf(cstrA.as_ptr(), self.totalAllocatedBytes());
    }
  }
}

impl Drop for AllocStats {
  fn drop(&mut self) {
    self.dump();
  }
}

struct DefaultAllocator {
  delegate: System,
}

impl DefaultAllocator {
  #[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
  pub const fn new() -> Self {
    Self {
      delegate: System,
    }
  }
}

unsafe impl GlobalAlloc for DefaultAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    #[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          stats.alloc(layout.size());
        }
      }
    }
    unsafe {
      return self.delegate.alloc(layout);
    }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    #[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          stats.free(layout.size());
        }
      }
    }
    unsafe {
      self.delegate.dealloc(ptr, layout);
    }
  }

  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    #[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          stats.allocZeroed(layout.size());
        }
      }
    }
    unsafe {
      return self.delegate.alloc_zeroed(layout);
    }
  }

  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8{
    #[cfg(all(feature="debugDefaultAllocator", feature="stats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          stats.realloc(layout.size(), new_size);
        }
      }
    }
    unsafe {
      return self.delegate.realloc(ptr, layout, new_size);
    }
  }
}

static GLOBAL_STATS: Mutex<Option<AllocStats>> = Mutex::new(None);

#[global_allocator]
static GLOBAL_ALLOCATOR: DefaultAllocator = DefaultAllocator::new();

fn main() {
  if let Ok(mut stats) = GLOBAL_STATS.lock() {
    *stats = Some(AllocStats::new(512*1024));
  }

  unsafe {
    if libc::atexit(process_cleanup_handler) != 0 {
      eprintln!("Failed to register atexit handler!");
      process::exit(1);
    }
  }

  let mut data = vec![1];
  for i in 1..=100 {
    data.push(i);
  }
  println!("data has {} elements", data.len());
  drop(data);

  process::exit(0);
}
