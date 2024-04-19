# box_raw_ptr

`box_raw_ptr` is a Rust library providing safe wrappers for working with raw pointers. These raw pointers are `*const T` and `*mut T`. These wrappers ensure memory safety by encapsulating the raw pointers in safe abstractions and providing safe methods for working with them.

## Description

box_raw_ptr is a Rust library that offers safe wrappers for working with raw pointers, specifically `*const T` and `*mut T`. These wrappers ensure memory safety by encapsulating the raw pointers within safe abstractions and providing methods that guarantee safe usage.

## Features

- **ConstRawPtr**: A wrapper for `*const T` providing methods for safely working with constant raw pointers.
- **MutRawPtr**: A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.

## Example

```rust
use box_raw_ptr::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};

fn main() {
    // External C++ Pointer Function Example:
    extern "C" {
        fn cpp_ptr() -> *mut i32;
    }

    // Get Unsafe External C++ Pointer
    let cpp_ptr: *mut i32 = unsafe { cpp_ptr() };
    
    // Convert Unsafe External C++ Pointer To MutRawPtr Of Type i32
    let mut mut_safe_ptr: MutRawPtr<i32> = MutRawPtr::new_mut_ptr(cpp_ptr);

    // Write To The Safe Pointer
    mut_safe_ptr.write_mut_ptr(20 as i32);

    // Print Value Of mut_safe_ptr Note: Uses .unwrap() as ptr is guaranteed not to be NULL
    println!("{}", t.unwrap_mut().unwrap());

    // Writing To MutRawPtr<T> Example:
    let mut_ptr: MutRawPtr<u8> = MutRawPtr::new_mut_ptr(&mut 13_u8 as *mut u8);

    // Cast MutRawPtr<T> To Option<*mut U>
    let u_ptr: *mut i32 = mut_ptr.mut_cast_ptr::<i32>().unwrap();

    match MutRawPtr::new_mut_ptr(u_ptr).write_mut_ptr(20 as i32) {
        Some(ptr) => {
            // Print MutRawPtr Memory Address
            println!("{}", ptr.mut_mem_addr());
        }
        None => (),
    }

    // Pointer Arithmetic For A [T; U] That Returns The Index Value In The Array Example:
    let arr: [i32; 5] = [1,2,3,4,5];

    // Create New ConstRawPtr<i32> From The Array As A Pointer
    let arr_ptr: ConstRawPtr<i32> = ConstRawPtr::new_const_ptr(arr.as_ptr());

    // Set The Index Of The Pointer
    ConstRawPtr::set_idx_ptr(&arr_ptr, 2)
        .inspect(|x| {
            let t: i32 = x.clone().unwrap_const().unwrap();
            // 2 Indexed From arr Equals 3
            assert_eq!(3, t);
        });
}
