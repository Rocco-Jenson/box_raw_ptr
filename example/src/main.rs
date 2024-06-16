use box_raw_ptr::mut_raw_ptr::MutRawPtr;

//Import C Function
#[link(name = "example", kind = "static")]
extern "C" {
    fn get_c_ptr() -> *mut i32;
}

fn main() {
    // Get int* from C and convert to MutRawPtr<i32>
    let ptr: MutRawPtr<i32> = MutRawPtr::mut_new_ptr(unsafe {
        get_c_ptr()
    });

    // Print memory address of C pointer and the underlying value
    println!("{:?} {}", ptr.memory_address(), ptr.unwrap_mut().unwrap());

    /*
    Memory is automatically dropped using free() from the box_raw_ptr Global Allocator
    See allocator.rs and allocator.c for implementation
    */
}
