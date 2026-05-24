use std::ffi::CStr;                                                                                                     
use std::cell::Cell;
use std::os::raw::c_char;                                                                                               
use std::alloc::{GlobalAlloc, Layout, System};

use super::stats::Stats;

unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

pub struct DefaultAllocator {
  delegate: System,
  #[cfg(all(feature="debugAllocatorStats"))]
  allocStats: Cell<Stats>,
}

impl DefaultAllocator {
  pub fn new(capacityBytes: usize) -> Self {
    debug_assert!(capacityBytes>0);
    Self {
      delegate: System,
      #[cfg(all(feature="debugAllocatorStats"))]
      allocStats: Stats::new(capacityBytes).into(),
    }
  }
}

unsafe impl GlobalAlloc for DefaultAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    // Panic if insufficient memory if stats enabled
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let stats = self.allocStats.take();
      if layout.size()>stats.freeBytes() {
        stats.dump();
        panic!("insufficient free space to allocate {} bytes", layout.size());
      }
    }

    // Try to allocate memory
    #[allow(unused_assignments)]
    let mut ptr: *mut u8 = 0 as *mut u8;
    unsafe {
      ptr = self.delegate.alloc(layout);
      if ptr == 0 as *mut u8 {
        panic!("allocation of {} bytes failed: {:?}", layout.size(), ptr);
      }
      #[cfg(feature="debugAllocatorTrace")]
      {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: alloc %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.take();
      stats.countAlloc(layout);
      self.allocStats.set(stats);
    }
  
    return ptr;
  }

  unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
    // Panic if insufficient memory if stats enabled
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let stats = self.allocStats.take();
      if layout.size()>stats.freeBytes() {
        stats.dump();
        panic!("insufficient free space to allocate {} bytes", layout.size());
      }
    }

    // Try to allocate memory
    #[allow(unused_assignments)]
    let mut ptr: *mut u8 = 0 as *mut u8;
    unsafe {
      ptr = self.delegate.alloc_zeroed(layout);
      if ptr == 0 as *mut u8 {
        panic!("allocation of {} bytes failed: {:?}", layout.size(), ptr);
      }
      #[cfg(feature="debugAllocatorTrace")]
      {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: allocZeroed %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.take();
      stats.countAlloc(layout);
      self.allocStats.set(stats);
    }
  
    return ptr;
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    // Free memory
    unsafe {
      self.delegate.dealloc(ptr, layout);
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: free %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.take();
      stats.countDealloc(layout);
      self.allocStats.set(stats);
    }
  }

  unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // Panic if insufficient memory if stats enabled
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let stats = self.allocStats.take();
      if new_size>layout.size() && (new_size-layout.size())>stats.freeBytes() {
        stats.dump();
        panic!("insufficient free space to resize {:?} from {} to {} bytes", ptr, layout.size(), new_size);
      }
    }

    // Try to resize memory
    #[allow(unused_assignments)]
    let mut newPtr: *mut u8 = 0 as *mut u8;
    unsafe {
      newPtr = self.delegate.realloc(ptr, layout, new_size);
      if newPtr == 0 as *mut u8 {
        panic!("allocation of {} bytes failed: {:?}", layout.size(), ptr);
      }
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: resize %p %lu to %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size(), newPtr, new_size);
      }
    }  

    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.take();
      stats.countRealloc(new_size, layout);
      self.allocStats.set(stats);
    }

    return newPtr;
  }
}
