#![allow(non_camel_case_types)]
use std::alloc::{GlobalAlloc, Layout, handle_alloc_error};
use self::c_ffi::{arch_type, c_void};

/* Custom C types to remove libc dependency */
mod c_ffi {
    #[cfg(target_pointer_width = "64")]
    pub type arch_type = u64;

    #[cfg(target_pointer_width = "32")]
    pub type arch_type = u32;

    pub type c_void = std::ffi::c_void;
}

#[link(name = "allocator", kind = "static")]
extern "C" {
    fn c_global_allocator(bytes: arch_type) -> *mut c_void;
    fn c_global_deallocator(ptr: *mut u8) -> c_void;
}

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

/*
Note pub(self): Only lib.rs uses C_GLOBAL_ALLOCATOR to make custom C allocations,
any other allocations in user project are defined with the
#[global_allocator] attribute
*/
#[global_allocator]
pub(self) static GLOBAL: C_GLOBAL_ALLOCATOR = C_GLOBAL_ALLOCATOR;
