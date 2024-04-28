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
        fn cpp_function() -> *const i32;
    }

    // Get Unsafe External C++ Pointer
    let cpp_ptr: *const i32 = unsafe { cpp_function() };
    
    // Convert Unsafe External C++ Pointer To MutRawPtr Of Type i32
    let safe_ptr: ConstRawPtr<i32> = ConstRawPtr::new_const_ptr(cpp_ptr);

    // Print Value Of safe_ptr if safe_ptr is not NULL
    // Note: .unwrap_const() returns Option<T>
    // if safe_ptr is not null returns Some(T)
    safe_ptr.unwrap_const().inspect(|ptr| {
        println!("{}", *ptr)
    });

    // Writing To MutRawPtr<T> Example:
    let mut_ptr: MutRawPtr<u8> = MutRawPtr::new_mut_ptr(&mut 13_u8 as *mut u8);

    // Cast MutRawPtr<T> To type U
    // Note: returns Option<*mut U>,
    // returns Some(*mut U) if not NULL
    let cast_ptr: *mut i32 = mut_ptr.mut_cast_ptr::<i32>().unwrap();

    // Writes to the mutable raw pointer
    // Note: returns Option<Self>,
    // returns Some(Self) if not NULL
    match MutRawPtr::new_mut_ptr(cast_ptr).write_mut_ptr(20 as i32) {
        Some(ptr) => {
            // Print MutRawPtr Memory Address
            println!("{}", ptr.mut_mem_addr());
        }
        None => (),
    }

    // Pointer Arithmetic For A [T; T] That Returns The Index Value In The Array Example:
    let arr: [i32; 5] = [1,2,3,4,5];

    // Create New ConstRawPtr<i32> From The Array As A Pointer
    let arr_ptr: ConstRawPtr<i32> = ConstRawPtr::new_const_ptr(arr.as_ptr());

    // Set The Index Of The Pointer
    // Note: .set_idx_ptr() 
    // returns None if idx is out of array bounds
    match arr_ptr.set_idx_ptr(2, &arr) {
        Some(safe_ptr) => {
            // offset of 2 from arr equals safe_ptr pointing to 3
            assert_eq!(3, safe_ptr.unwrap_const().unwrap())
        }
        None => ()
    }
}