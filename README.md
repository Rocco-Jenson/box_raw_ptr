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
use box_raw_ptr::mut_raw_ptr::MutRawPtr;

#[link(name = "example", kind = "static")]
extern "C" {
    fn c_ptr() -> *mut Data;
    fn c_ptr2() -> *mut std::ffi::c_int;
}

#[repr(C)]
#[derive(Clone, Copy)] /* DATA TYPE MUST HAVE COPY AND CLONE TRAITS */
struct Data {
    a: i32,
    b: f64,
}

fn main() {
    // Example: Import C pointer and write to the allocated data
    let mut safeptr: MutRawPtr<Data> = MutRawPtr::new(unsafe { c_ptr() }, /*# of Data Blocks*/ 1, /*offset*/ 0);

    assert_eq!(16, safeptr.size_of());

    safeptr.write_ptr(Data {a: 100, b: 12.0});

    assert_eq!(100, (safeptr.access().unwrap()).a);

    // Example: Iteratively Rewrite Values in a Block Of Data (Assuming 5 Blocks of i32)
    let mut safeptr: MutRawPtr<_> = MutRawPtr::new( unsafe { c_ptr2() }, 5, 0);

    for _ in 0..=4 {
        safeptr.change_offset(i).unwrap();
        safeptr.write_ptr(100 as i32).unwrap();
        println!("{}", safeptr.access().unwrap());
    }
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

box_raw_ptr = "2.1.0"

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
