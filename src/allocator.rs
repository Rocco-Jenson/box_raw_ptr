#![allow(non_camel_case_types)]

use std::alloc::{GlobalAlloc, Layout, handle_alloc_error};

/* Custom C types to remove libc dependency */
#[cfg(target_pointer_width = "64")]
type arch_type = u64;

#[cfg(target_pointer_width = "32")]
type arch_type = u32;

type c_void = std::ffi::c_void;

#[link(name = "allocator", kind = "static")]
extern "C" {
    fn c_global_allocator(bytes: arch_type) -> *mut c_void;
    fn c_global_deallocator(ptr: *mut u8) -> c_void;
}

/*
Note: Only lib.rs uses C_GLOBAL_ALLOCATOR to make custom C allocations,
any other allocations in user project are defined with the
#[global_allocator] attribute
*/
pub(self) struct C_GLOBAL_ALLOCATOR;

unsafe impl GlobalAlloc for C_GLOBAL_ALLOCATOR {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr: *mut u8 = c_global_allocator(layout.size() as arch_type) as *mut u8;    
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if !ptr.is_null() {
            c_global_deallocator(ptr);
        }
    }
}

#[global_allocator]
pub(self) static GLOBAL: C_GLOBAL_ALLOCATOR = C_GLOBAL_ALLOCATOR;