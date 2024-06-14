#![allow(non_camel_case_types)]
use std::alloc::{GlobalAlloc, Layout, handle_alloc_error};

/* C Types */
type size_t = usize;
type c_void = std::ffi::c_void;

extern "C" {
    fn c_global_allocator(bytes: size_t) -> *mut c_void;
    fn c_global_deallocator(ptr: *mut c_void) -> c_void;
}

#[allow(non_camel_case_types)]
struct C_GLOBAL_ALLOCATOR;

unsafe impl GlobalAlloc for C_GLOBAL_ALLOCATOR {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr: *mut u8 = c_global_allocator(layout.size()) as *mut u8;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if !ptr.is_null() {
            c_global_deallocator(ptr as *mut c_void);
        }
    }
}

#[global_allocator]
static GLOBAL: C_GLOBAL_ALLOCATOR = C_GLOBAL_ALLOCATOR;
