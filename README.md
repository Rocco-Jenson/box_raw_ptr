# box_raw_ptr

box_raw_ptr is a Rust library providing safe abstractions for working with raw pointers (`*const T` and `*mut T`). It ensures proper alignment, bounds checking, and safe memory operations, inspired by Rust's safety principles while allowing interoperability with C-style memory management.

## Features

- **Type Safety**: Wrappers (`ConstRawPtr` and `MutRawPtr`) ensure safe usage of raw pointers (`*const T` and `*mut T`).

- **Bounds Checking**: Methods to check and adjust offsets within allocated memory.

- **Alignment Guarantees**: Ensures pointers are aligned according to `T`.

- **Memory Management**: Includes methods for deallocating memory and safely handling null pointers.

- **Interoperability**: Facilitates safe interaction with memory allocated by C functions or Rust's allocator.

## Components

- **ConstRawPtr**: Provides safe operations on `*const T` pointers, including bounds checking and memory release.

- **MutRawPtr**: Offers safe operations on `*mut T` pointers, supporting mutable access and memory management.

## Usage

```rust
use box_raw_ptr::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};

fn main() {
    // Example: Import C pointer and write to the allocated data
    #[link(name = "example", kind = "static")]
    extern "C" {
        fn c_ptr() -> *mut std::ffi::c_int;
    }

    let ptr: *mut i32 = unsafe { c_ptr() };

    let safeptr = MutRawPtr::new(ptr, 1, 1);

    safeptr.write_ptr(14).unwrap();

    assert_eq!(safeptr.unwrap().unwrap(), 14)

    // Example: Create a ConstRawPtr to safely handle a raw const pointer
    // Allocate properly aligned memory for an i32
    let alloc: *const i32 = unsafe { 
    std::alloc::alloc(std::alloc::Layout::from_size_align(20, 4).unwrap()) as *const i32 
    };
    let mut ptr: ConstRawPtr<i32> = ConstRawPtr::new(alloc, 5, 1);

    ptr.change_offset(4).unwrap();

    println!("{} : {}", ptr.unwrap().unwrap(), ptr.memory_address());
}
```

## Safety Considerations

- **Unsafe Contexts**: Use of raw pointers inherently involves unsafe operations.

- **Memory Safety**: Ensure proper initialization and alignment of pointers.

- **Dropping Pointers**: Manually dropping pointers can lead to undefined behavior if used afterward.

## Installation

Add the following to your `Cargo.toml`:

```toml

[dependencies]

box_raw_ptr = "2.0.0"

```

## Documentation

For detailed API documentation, refer to [docs.rs](https://docs.rs/box_raw_ptr/latest/box_raw_ptr/).

## License

MIT License

Copyright (c) [2024] [Rocco Zinedine Samuel Jenson]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
