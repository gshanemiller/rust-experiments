use std::ffi::CStr;                                                                                                     
use std::cell::Cell;
use std::os::raw::c_char;                                                                                               
use std::ptr::{self, NonNull};
use std::alloc::{Allocator, AllocError, Layout, System, GlobalAlloc};

use super::stats::Stats;

unsafe extern "C" {
  fn printf(format: *const c_char, ...) -> i32;
}

#[allow(non_snake_case)]
pub struct DefaultAllocator {
  delegate: System,
  #[cfg(all(feature="debugAllocatorStats"))]
  allocStats: Cell<Stats>,
}

#[allow(non_snake_case)]
impl DefaultAllocator {
  pub fn new(capacityBytes: usize) -> Self {
    debug_assert!(capacityBytes>0);
    Self {
      delegate: System,
      #[cfg(all(feature="debugAllocatorStats"))]
      allocStats: Stats::new(capacityBytes).into(),
    }
  }

  pub fn dump(&self) {
    #[cfg(all(feature="debugAllocatorStats"))] {
      let stats = self.allocStats.get();
      stats.dump();
    }
  }
}

impl Drop for DefaultAllocator {                                                                                              
  fn drop(&mut self) {                                                                                                  
   let stats = self.allocStats.get();
   stats.dump();
  }                                                                                                                     
}

#[allow(non_snake_case)]
unsafe impl Allocator for DefaultAllocator {
  fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    // Panic if insufficient memory if stats enabled
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let stats = self.allocStats.get();
      if layout.size()>stats.freeBytes() {
        stats.dump();
        panic!("insufficient free space to allocate {} bytes", layout.size());
      }
    }

    // Try to allocate memory or panic
    #[allow(unused_assignments)]
    let mut ptr = 0 as *mut u8;
    unsafe {
      ptr = self.delegate.alloc(layout);
      // Panic if bad memory
      if ptr == 0 as *mut u8 {
        #[cfg(all(feature="debugAllocatorStats"))]
        {
          let stats = self.allocStats.get();
          stats.dump();
        }
        panic!("failed to alloc {} bytes: got zero pointer {:?}", layout.size(), ptr);
      }
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: alloc %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.get();
      stats.countAlloc(layout);
      self.allocStats.set(stats);
    }
  
    // Good lord! This Rust noise
    let slice_ptr: *mut [u8] = ptr::slice_from_raw_parts_mut(ptr, layout.size());
    let non_null_slice = unsafe { NonNull::new_unchecked(slice_ptr) };
    Ok(non_null_slice)
  }

  fn allocate_zeroed(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    // Panic if insufficient memory if stats enabled
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let stats = self.allocStats.get();
      if layout.size()>stats.freeBytes() {
        stats.dump();
        panic!("insufficient free space to allocate {} bytes", layout.size());
      }
    }

    // Try to allocate memory or panic
    #[allow(unused_assignments)]
    let mut ptr = 0 as *mut u8;
    unsafe {
      ptr = self.delegate.alloc_zeroed(layout);
      // Panic if bad memory
      if ptr == 0 as *mut u8 {
        #[cfg(all(feature="debugAllocatorStats"))]
        {
          let stats = self.allocStats.get();
          stats.dump();
        }
        panic!("failed to alloc {} bytes: got zero pointer {:?}", layout.size(), ptr);
      }
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: allocZeroed %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), ptr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.get();
      stats.countAlloc(layout);
      self.allocStats.set(stats);
    }

    // Good lord! This Rust noise
    let slice_ptr: *mut [u8] = ptr::slice_from_raw_parts_mut(ptr, layout.size());
    let non_null_slice = unsafe { NonNull::new_unchecked(slice_ptr) };
    Ok(non_null_slice)
  }

  unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    let rawPtr = ptr.as_ptr();

    // Free memory
    unsafe {
      self.delegate.dealloc(rawPtr, layout);
    }

    #[cfg(feature="debugAllocatorTrace")]
    {
      unsafe {
        let cstr: &CStr = CStr::from_bytes_with_nul(b"dfltAlloc: free %p %lu bytes\n\0").unwrap();
        printf(cstr.as_ptr(), rawPtr, layout.size());
      }
    }

    // Book keeping
    #[cfg(all(feature="debugAllocatorStats"))]
    {
      let mut stats = self.allocStats.get();
      stats.countDealloc(layout);
      self.allocStats.set(stats);
    }
  }
}
