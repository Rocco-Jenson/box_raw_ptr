use std::alloc::{GlobalAlloc, Layout, handle_alloc_error};

/* C Types to remove libc dependency */
#[cfg(target_pointer_width = "64")]
type arch_bytes = u64;

#[cfg(target_pointer_width = "32")]
type arch_bytes = u32;

type c_void = std::ffi::c_void;

extern "C" {
    fn c_global_allocator(bytes: arch_bytes) -> *mut c_void;
    fn c_global_deallocator(ptr: *mut c_void) -> c_void;
    fn global_allocation_info() -> arch_bytes; 
}

#[allow(non_camel_case_types)]
struct C_GLOBAL_ALLOCATOR;

unsafe impl GlobalAlloc for C_GLOBAL_ALLOCATOR {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr: *mut u8 = c_global_allocator(layout.size()) as *mut u8;
        let heap_count: arch_bytes = global_allocation_info();
        
        if ptr.is_null() || heap_count < 0 {
            handle_alloc_error(layout);
        }
        
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        /* Ptr null check is done in function */
        c_global_deallocator(ptr as *mut c_void);
    }
}

#[global_allocator]
static GLOBAL: C_GLOBAL_ALLOCATOR = C_GLOBAL_ALLOCATOR;
