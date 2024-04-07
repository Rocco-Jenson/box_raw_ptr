# box_raw_ptr

`box_raw_ptr` is a Rust library providing safe wrappers for working with raw pointers, specifically `*const T` and `*mut T`. These wrappers ensure memory safety by encapsulating the raw pointers in safe abstractions and providing safe methods for working with them.

## Description

box_raw_ptr is a Rust library that offers safe wrappers for working with raw pointers, specifically `*const T` and `*mut T`. These wrappers ensure memory safety by encapsulating the raw pointers within safe abstractions and providing methods that guarantee safe usage.

## Features

- **ConstRawPtr**: A wrapper for `*const T` providing methods for safely working with constant raw pointers.
- **MutRawPtr**: A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.

## Example

```rust
use box_raw_ptr::const_raw_ptr::ConstRawPtr;

fn main() {
    // Create New Const Pointer
    let raw_ptr = ConstRawPtr::new_const_ptr(1 as *const i32);

    // Print the memory address of the raw pointer
    raw_ptr.print_const_ptr(true);

    // Print the value pointed to by the raw pointer
    raw_ptr.print_const_ptr(false);

    // Unwrap and Deref Box<*const T> To Option<T>
    raw_ptr.unwrap_const();

    // Create const NULL pointer
    let null_ptr = ConstRawPtr::const_null_ptr();

    // Unwrap Box<*const T> To *const T to check if NULL
    if null_ptr.is_null() {
        println!("NULL PTR")
    }
}