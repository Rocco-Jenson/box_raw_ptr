# box_raw_ptr

`box_raw_ptr` is a Rust library designed to provide safe wrappers for handling raw pointers `*const T` and `*mut T`. The library ensures memory safety by encapsulating these raw pointers within safe abstractions, leveraging Rust's ownership and borrowing rules.

All heap allocations in this library utilize `malloc` and `free`, ensuring compatibility with C FFI (Foreign Function Interface). This approach allows seamless integration with C while maintaining safety through Rust's type system and ownership model. The raw pointers defined in Rust that are not imported from C will still be compatable.

The `Box<T>` wrapper around the raw pointers by `box_raw_ptr` ensures that operations on raw pointers are safe and aligned with Rust's memory safety principles. This makes it easier to work with raw pointers in Rust codebases, especially when interfacing with C libraries or performing low-level memory management tasks.

By combining the power of Rust's safety features with the flexibility of raw pointers, `box_raw_ptr` facilitates robust and secure memory management in Rust applications.

## Features

- **ConstRawPtr**: A wrapper for `*const T` providing methods for safely working with constant raw pointers.
- **MutRawPtr**: A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.

## Example

```rust
use box_raw_ptr::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};

fn main() {
    // External C Pointer Function Example:
    #[link(name = "example")]
    extern "C" {
        fn get_c_ptr() -> *const libc::c_int;
    }

    // Get Unsafe External C Pointer
    let c_ptr: *const i32 = unsafe { get_c_ptr() };

    // Convert Unsafe External C Pointer To ConstRawPtr Of Type i32
    let safe_ptr: ConstRawPtr<i32> = ConstRawPtr::const_new_ptr(c_ptr);

    // Print Value Of safe_ptr if safe_ptr is not NULL
    // Note: .unwrap_const() returns Option<T>
    // if safe_ptr is not null returns Some(T)
    if let Some(ptr) = safe_ptr.unwrap_const() {
        println!("{}", ptr);
    }

    // Writing To MutRawPtr<T> Example:
    let mut value: u8 = 13;
    let mut_ptr: MutRawPtr<u8> = MutRawPtr::mut_new_ptr(&mut value as *mut u8);

    // Cast MutRawPtr<T> To type U
    // Note: returns Option<*mut U>,
    // returns Some(*mut U) if not NULL
    if let Some(cast_ptr) = mut_ptr.cast_ptr::<i32>() {
        // Writes to the mutable raw pointer
        // Note: returns Option<Self>,
        // returns Some(Self) if not NULL
        if let Some(ptr) = MutRawPtr::mut_new_ptr(cast_ptr).write_ptr(20) {
            // Print MutRawPtr Memory Address
            println!("{}", ptr.memory_address());
        }
    }

    // Pointer Arithmetic For A [T; T] That Returns The Index Value In The Array Example:
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Create New ConstRawPtr<i32> From The Array As A Pointer
    let arr_ptr: ConstRawPtr<i32> = ConstRawPtr::const_new_ptr(arr.as_ptr());

    // Set The Index Of The Pointer
    // Note: .set_idx_ptr()
    // returns None if idx is out of array bounds
    if let Some(safe_ptr) = arr_ptr.set_idx_ptr(2, &arr) {
        // offset of 2 from arr equals safe_ptr pointing to 3
        assert_eq!(3, safe_ptr.unwrap_const().unwrap());
    }
}
