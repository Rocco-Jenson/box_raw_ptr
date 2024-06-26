use box_raw_ptr::mut_raw_ptr::MutRawPtr;

/* 
Import C file
NOTE: Correctly specify type of file (ex: kind = "dylib" || "static") or linker will throw error 
*/
#[link(name = "example", kind = "static")]
extern "C" {
    fn get_c_ptr() -> *mut i32;
}

fn main() {
    /* Get int* from C and convert to MutRawPtr<i32> */
    let ptr: *mut i32 = unsafe {
        get_c_ptr()
    };

    let safe_ptr: MutRawPtr<i32> = MutRawPtr::new(ptr, 1, 1);

    /* Print memory address of C pointer and the underlying value */
    println!("{} : {}", safe_ptr.memory_address(), safe_ptr.unwrap().unwrap());

    /* 
    Memory is automatically dropped using free() from the box_raw_ptr Global Allocator 
    See allocator.rs and allocator.c for implementation 
    */
}
