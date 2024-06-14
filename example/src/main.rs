use box_raw_ptr::mut_raw_ptr::MutRawPtr;

#[link(name = "example")]
extern "C" {
    fn get_c_ptr() -> *mut libc::c_int;
}

fn main() {
    let mut mut_ptr: MutRawPtr<i32> = MutRawPtr::mut_new_ptr(unsafe {
        get_c_ptr()
    });

    mut_ptr.write_ptr(32);

    println!("{:?} {}", mut_ptr.memory_address(), mut_ptr.unwrap_mut().unwrap());
}
