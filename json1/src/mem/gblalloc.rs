#[cfg(feature="debugAllocatorTrace")]
use std::ffi::CStr;
#[cfg(feature="debugAllocatorTrace")]
use std::os::raw::c_char;

use super::stats::Stats;
use super::gblstats::GlobalStats;
use std::alloc::{Layout, System, GlobalAlloc};

#[cfg(feature="debugAllocatorTrace")]
unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

#[allow(non_snake_case)]
pub struct GlobalAllocator {
  delegate: System,
  stats: GlobalStats,
}

#[allow(non_snake_case)]
impl GlobalAllocator {
  pub const fn new(capacityBytes: usize) -> Self {
    debug_assert!(capacityBytes>0);
    Self {
      delegate: System,
      stats: GlobalStats::new(capacityBytes),
    }
  }

  pub fn stats(&self) -> Stats {
    let stats = self.stats.lock().unwrap();
    let ret = *stats;
    return ret;
  }
}

unsafe impl GlobalAlloc for GlobalAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    debug_assert!(layout.size()>0);

    // Obtain locked guard on shared global allocator stats
    let mut stats = self.stats.lock().unwrap();

    // Panic if insufficient memory if stats enabled
    if layout.size()>stats.freeBytes() {
      stats.dump("GlobalAllocator");
      panic!("insufficient free space to allocate {} bytes", layout.size());
    }

    // Try to allocate memory or panic
    #[allow(unused_assignments)]
    let mut ptr = 0 as *mut u8;
    unsafe {
      ptr = self.delegate.alloc(layout);
      // Panic if bad memory
      if ptr == 0 as *mut u8 {
        stats.dump("GlobalAllocator");
        drop(stats);
        panic!("failed to alloc {} bytes: got zero pointer {:?}", layout.size(), ptr);
      }
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"gblAlloc: alloc %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    stats.countAlloc(layout.size());

    // Return
    return ptr;
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    debug_assert!(layout.size()>0);

    // Obtain locked guard on shared global allocator stats
    let mut stats = self.stats.lock().unwrap();

    // Global allocators have a global init order problem. have observed mismatched alloc/frees
    if layout.size()<=stats.allocatedBytes() {
      unsafe {
        self.delegate.dealloc(ptr, layout);
      }
      stats.countDealloc(layout.size());
    } else {
      stats.dump("GlobalAllocator");
      drop(stats);
      panic!("internal error: cannot free {:?} {} bytes: underflow in allocatedBytes", ptr, layout.size());
    }

    #[cfg(all(feature="debugAllocatorTrace"))]
    {
      let cstr: &CStr = CStr::from_bytes_with_nul(b"gblAlloc: free %p %lu bytes\n\0").unwrap();
      unsafe {
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }
  }
}
