use std::process;
use std::ffi::CStr;
use std::sync::Mutex;
use std::os::raw::c_char;
use std::alloc::{GlobalAlloc, Layout, System};

unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

#[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
extern "C" fn destroyAllocStats() {
  if let Ok(mut guard) = GLOBAL_STATS.lock() {
    *guard = None;
  }
}

#[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
#[allow(non_snake_case)]
struct AllocStats {
  d_capacityBytes:        usize,
  d_allocatedBytes:       usize,
  d_maxAllocatedBytes:    usize,
  d_freeCount:            usize,
  d_allocCount:           usize,
  d_totalFreedBytes:      usize,
  d_totalAllocatedBytes:  usize,
}

#[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
#[allow(non_snake_case)]
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

  pub fn alloc(&mut self, layout: Layout, delegate: System) -> *mut u8 {
    // Panic if insufficient memory
    if layout.size()>self.freeBytes() {
      self.dump();
      panic!("cannot alloc {} bytes: insufficient free space", layout.size());
    }

    #[allow(unused_assignments)]
    let mut ptr = 0 as *mut u8;
    unsafe {
      ptr = delegate.alloc(layout);
      // Panic if bad memory
      if ptr == 0 as *mut u8 {
        self.dump();
        panic!("failed to alloc {} bytes: got zero pointer {:?}", layout.size(), ptr);
      }
    }

    // Book keeping
    self.d_allocCount += 1;
    self.d_allocatedBytes += layout.size();
    self.d_totalAllocatedBytes += layout.size();
    if self.d_allocatedBytes>self.d_maxAllocatedBytes {
      self.d_maxAllocatedBytes = self.d_allocatedBytes;
    }

    #[cfg(all(feature="debugAllocatorTrace"))]
    {
      let cstr: &CStr = CStr::from_bytes_with_nul(b"alloc %p %lu bytes\n\0").unwrap();
      unsafe {
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    return ptr;
  }

  pub fn allocZeroed(&mut self, layout: Layout, delegate: System) -> *mut u8 {
    // Panic if insufficient memory
    if layout.size()>self.freeBytes() {
      self.dump();
      panic!("cannot alloc {} bytes: insufficient free space", layout.size());
    }

    #[allow(unused_assignments)]
    let mut ptr = 0 as *mut u8;
    unsafe {
      ptr = delegate.alloc_zeroed(layout);
      // Panic if bad memory
      if ptr == 0 as *mut u8 {
        self.dump();
        panic!("failed to alloc {} bytes: got zero pointer {:?}", layout.size(), ptr);
      }
    }

    // Book keeping
    self.d_allocCount += 1;
    self.d_allocatedBytes += layout.size();
    self.d_totalAllocatedBytes += layout.size();
    if self.d_allocatedBytes>self.d_maxAllocatedBytes {
      self.d_maxAllocatedBytes = self.d_allocatedBytes;
    }

    #[cfg(all(feature="debugAllocatorTrace"))]
    {
      let cstr: &CStr = CStr::from_bytes_with_nul(b"allocZeroed %p %lu bytes\n\0").unwrap();
      unsafe {
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    return ptr;
  }

  pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout, delegate: System) {
    // Book keeping
    if layout.size()<=self.d_allocatedBytes {
      self.d_freeCount += 1;
      self.d_allocatedBytes -= layout.size();
      self.d_totalFreedBytes += layout.size();
      unsafe {
        delegate.dealloc(ptr, layout);
      }
    } else {
      let cstr1: &CStr = CStr::from_bytes_with_nul(b"warn: invalid free %p %lu bytes (stats not updated\n\0").unwrap();
      unsafe {
        printf(cstr1.as_ptr(), ptr, layout.size());
        delegate.dealloc(ptr, layout);
      }
    }

    #[cfg(all(feature="debugAllocatorTrace"))]
    {
      let cstr: &CStr = CStr::from_bytes_with_nul(b"free %p %lu bytes\n\0").unwrap();
      unsafe {
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }
  }

  pub fn realloc(&mut self, ptr: *mut u8, new_size: usize, layout: Layout, delegate: System) -> *mut u8 {
    let mut bigger = true;
    let mut delta: usize = 0;
    #[allow(unused_assignments)]
    let mut newPtr = 0 as *mut u8;

    if new_size==layout.size() {
      // theoretically this should do nothing, but do it anyway. stats unchanged
      unsafe {
        newPtr = delegate.realloc(ptr, layout, new_size);
      }
    } else if new_size>layout.size() {
      delta = new_size-layout.size();
      if delta>self.freeBytes() {
        self.dump();
        panic!("cannot resize {:?} from {} to {} bytes: insufficient space", ptr, layout.size(), new_size);
      }
      unsafe {
        newPtr = delegate.realloc(ptr, layout, new_size);
      }
    } else {
      bigger = false;
      delta = layout.size() - new_size;
      unsafe {
        newPtr = delegate.realloc(ptr, layout, new_size);
      }
    }

    // Panic if bad pointer
    if newPtr == 0 as *mut u8 {
      self.dump();
      panic!("failed to resize {:?} from {} to {} bytes: got zero pointer {:?}", ptr, layout.size(), new_size, newPtr);
    }

    // Book keeping
    if bigger {
      // allocated memory
      self.d_allocCount += 1;
      self.d_allocatedBytes += delta;
      self.d_totalAllocatedBytes += delta;
      if self.d_allocatedBytes>self.d_maxAllocatedBytes {
        self.d_maxAllocatedBytes = self.d_allocatedBytes;
      }
    } else {
      // freed memory
      self.d_freeCount += 1;
      self.d_allocatedBytes -= delta;
      self.d_totalFreedBytes += delta; 
    }

    #[cfg(all(feature="debugAllocatorTrace"))]
    {
      let cstr: &CStr = CStr::from_bytes_with_nul(b"resize %p to %p from %lu to %lu bytes\n\0").unwrap();
      unsafe {
        printf(cstr.as_ptr(), ptr, newPtr, layout.size(), new_size);
      }
    }

    return newPtr;
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

#[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
impl Drop for AllocStats {
  fn drop(&mut self) {
    self.dump();
  }
}

#[cfg(all(feature="debugAllocator"))]
struct DefaultAllocator {
  delegate: System,
}

#[cfg(all(feature="debugAllocator"))]
impl DefaultAllocator {
  pub const fn new() -> Self {
    Self {
      delegate: System,
    }
  }
}

#[cfg(all(feature="debugAllocator"))]
unsafe impl GlobalAlloc for DefaultAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    #[cfg(not(feature="debugAllocatorStats"))]
    {
      unsafe {
        let ptr = self.delegate.alloc(layout);
        let cstr: &CStr = CStr::from_bytes_with_nul(b"alloc %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
        return ptr;
      }
    }
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          return stats.alloc(layout, self.delegate);
        }
      }
    }
  }

  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    #[cfg(not(feature="debugAllocatorStats"))]
    {
      unsafe {
        let ptr = self.delegate.alloc_zeroed(layout);
        let cstr: &CStr = CStr::from_bytes_with_nul(b"alloc_zeroed %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
        return ptr;
      }
    }
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          return stats.allocZeroed(layout, self.delegate);
        }
      }
    }
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    #[cfg(not(feature="debugAllocatorStats"))]
    {
      unsafe {
        self.delegate.dealloc(ptr, layout);
        let cstr: &CStr = CStr::from_bytes_with_nul(b"free %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
        return;
      }
    }
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          stats.dealloc(ptr, layout, self.delegate);
        }
      }
      return;
    }
  }

  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    #[cfg(not(feature="debugAllocatorStats"))]
    {
      unsafe {
        let ptr = self.delegate.realloc(ptr, layout, new_size);
        let cstr: &CStr = CStr::from_bytes_with_nul(b"resize %p %lu to %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size(), new_size);
        return ptr;
      }
    }
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      if let Ok(mut guard) = GLOBAL_STATS.lock() {
        if let Some(stats) = guard.as_mut() {
          return stats.realloc(ptr, new_size, layout, self.delegate);
        }
      }
    }
  }
}

#[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
static GLOBAL_STATS: Mutex<Option<AllocStats>> = Mutex::new(None);

#[cfg(all(feature="debugAllocator"))]
#[global_allocator]
static GLOBAL_ALLOCATOR: DefaultAllocator = DefaultAllocator::new();

fn main() {
  #[cfg(all(feature="debugAllocator", feature="debugAllocatorStats"))]
  {
    if let Ok(mut stats) = GLOBAL_STATS.lock() {
      *stats = Some(AllocStats::new(512*1024));
    }

    unsafe {
      if libc::atexit(destroyAllocStats) != 0 {
        eprintln!("Failed to register atexit handler 'destroyAllocStats'");
        process::exit(1);
      }
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
