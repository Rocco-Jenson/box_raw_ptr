//! # box_raw_ptr
//!
//! box_raw_ptr is a Rust library providing safe abstractions for working with raw pointers (`*const T` and `*mut T`). It ensures proper alignment, bounds checking, and safe memory operations, inspired by Rust's safety principles while allowing interoperability with C-style memory management.
//!
//! ## Features
//!
//! - **Type Safety**: Wrappers (`ConstRawPtr` and `MutRawPtr`) ensure safe usage of raw pointers (`*const T` and `*mut T`).
//!
//! - **Bounds Checking**: Methods to check and adjust offsets within allocated memory.
//!
//! - **Alignment Guarantees**: Ensures pointers are aligned according to `T`.
//!
//! - **Memory Management**: Includes methods for deallocating memory and safely handling null pointers.
//!
//! - **Interoperability**: Facilitates safe interaction with memory allocated by C functions or Rust's allocator.
//!
//! ## Components
//!
//! - **ConstRawPtr**: Provides safe operations on `*const T` pointers, including bounds checking and memory release.
//!
//! - **MutRawPtr**: Offers safe operations on `*mut T` pointers, supporting mutable access and memory management.
//!
//! ## Usage
//!
//! ```rust
//! use box_raw_ptr::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};
//!
//! fn main() {
//!     // Example: Import C pointer and write to the allocated data
//!     #[link(name = "example", kind = "static")]
//!     extern "C" {
//!         fn c_ptr() -> *mut std::ffi::c_int;
//!     }
//!
//!     let ptr: *mut i32 = unsafe { c_ptr() };
//!
//!     let safeptr = MutRawPtr::new(ptr, 1, 1);
//!
//!     safeptr.write_ptr(14).unwrap();
//!
//!     assert_eq!(safeptr.unwrap().unwrap(), 14)
//!
//!     // Example: Create a ConstRawPtr to safely handle a raw const pointer
//!     // Allocate properly aligned memory for an i32
//!     let alloc: *const i32 = unsafe { 
//!         std::alloc::alloc(std::alloc::Layout::from_size_align(20, 4).unwrap()) as *const i32 
//!     };
//!     let mut ptr: ConstRawPtr<i32> = ConstRawPtr::new(alloc, 5, 1);
//!
//!     ptr.change_offset(4).unwrap();
//!
//!     println!("{} : {}", ptr.unwrap().unwrap(), ptr.memory_address());
//! }
//! ```
//!
//! ## Safety Considerations
//!
//! - **Unsafe Contexts**: Use of raw pointers inherently involves unsafe operations.
//!
//! - **Memory Safety**: Ensure proper initialization and alignment of pointers.
//!
//! - **Dropping Pointers**: Manually dropping pointers can lead to undefined behavior if used afterward.
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! box_raw_ptr = "2.0.0"
//! ```
//!
//! ## Documentation
//!
//! For detailed API documentation, refer to [docs.rs](https://docs.rs/box_raw_ptr/latest/box_raw_ptr/).
//!
//! ## License
//!
//! MIT License
//!
//! Copyright (c) 2024 Rocco Zinedine Samuel Jenson
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.

/* 
Imports C_Global_Allocator to library
See allocator.rs and allocator.c for implementation
Note: ALL LIBRARY HEAP ALLOCATIONS MANAGED BY MALLOC AND FREE
*/
mod allocator;

pub mod const_raw_ptr {
    use std::marker::{Copy, Send, Sync};

    /// A wrapper for `*const T` providing methods for safely working with constant raw pointers.
    /// 
    /// `ConstRawPtr` ensures that the raw pointer is properly aligned and provides utility methods
    /// for checking bounds, changing offsets, and other common pointer operations. 
    ///
    /// Fields:
    /// - `ptr: *const T`: A raw constant pointer to the data.
    /// - `memory_length: usize`: The length of the memory block that `ptr` points to.
    /// - `offset: usize`: The current position within the memory block.
    ///
    /// Notes:
    /// - `memory_length` is not zero-based indexed.
    /// - `offset` is not zero-based indexed.
    ///
    /// # Safety
    ///
    /// Working with raw pointers is inherently unsafe. Ensure that the memory pointed to by `ptr` is valid 
    /// and properly aligned before using this struct.
    pub struct ConstRawPtr<T> 
    where  T: Sized + Copy + Send + Sync
    {
        ptr: *const T,
        memory_length: usize,
        offset: usize,
    }

    impl<T: Sized + Copy + Send + Sync> ConstRawPtr<T> {
        /// Creates a new `ConstRawPtr` with the given pointer, memory length, and offset.
        /// 
        /// This method ensures that the pointer is properly aligned and that the offset is within the bounds 
        /// of the allocated memory length.
        /// 
        /// # Panics
        /// 
        /// Panics if the pointer is not aligned to `T` or if the offset is not within the bounds of the memory length.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let alloc_ptr: *const i32 = ...; // Assume this is a properly allocated and aligned pointer either from C or using Rust's std::alloc::alloc and std::alloc::Layout otherwise it will panic.
        /// let ptr = ConstRawPtr::new(alloc_ptr, 1, 1);
        /// ```
        #[inline]
        pub fn new(ptr: *const T, memory_length: usize, offset: usize) -> Self {
            assert!((ptr as usize) % std::mem::align_of::<T>() == 0, "box_raw_ptr Err: Memory Not Aligned");
            assert!(offset <= memory_length && offset > 0, "box_raw_ptr Err: Offset Is Not Within Bounds");
            Self { ptr, memory_length, offset, }
        }

        /// Creates a new `ConstRawPtr` with a null pointer and zero memory length and offset.
        /// 
        /// This is useful for creating a placeholder `ConstRawPtr` that can later be assigned a valid pointer.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let null_ptr = ConstRawPtr::<i32>::nullptr();
        /// ```
        #[inline]
        pub fn nullptr() -> Self {
            Self { ptr: std::ptr::null(), memory_length: 0, offset: 0 }
        }

        /// Manually drops the `ConstRawPtr` instance.
        /// 
        /// # Safety
        /// 
        /// This function is unsafe because it drops the instance manually, which can lead to undefined behavior 
        /// if the instance is used after being dropped.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// unsafe {
        ///     ptr.manual_drop();
        /// }
        /// ```
        #[inline]
        pub unsafe fn manual_drop(self) -> () {
            drop(self);
        }

        /// Checks if the current offset is within the bounds of the memory length.
        /// 
        /// This method ensures that the pointer is pointing to a valid position within the allocated memory block.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// assert!(ptr.check_bounds());
        /// ```
        #[inline]
        pub fn check_bounds(&self) -> bool {
            (1..=self.memory_length).contains(&self.offset)
        }

        /// Checks if the pointer is not null and properly aligned.
        /// 
        /// This method ensures that the pointer is valid and meets the alignment requirements of `T`.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// assert!(ptr.check_ptr());
        /// ```
        pub fn check_ptr(&self) -> bool {
            if self.ptr.is_null() {
                return false;
            }
            let align: usize = std::mem::align_of::<T>();
            (self.ptr as usize) % align == 0
        }

        /// Returns the current offset.
        /// 
        /// This method provides the current offset within the memory block.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let offset = ptr.check_offset();
        /// ```
        pub fn check_offset(&self) -> usize {
            self.offset
        }

        /// Returns the current memory length.
        /// 
        /// This method provides the total length of the memory block that the pointer points to.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let length = ptr.check_memory_length();
        /// ```
        pub fn check_memory_length(&self) -> usize {
            self.memory_length
        }

        /// Changes the offset by a given index, if the resulting offset is within bounds.
        /// 
        /// This method allows you to move the pointer by a specified index within the memory block, 
        /// ensuring that the new offset is within bounds.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// assert!(ptr.change_offset(2).is_some());
        /// ```
        pub fn change_offset(&mut self, index: isize) -> Option<()> {
            if !self.check_ptr() {
                return None;
            }
            let new_offset: isize = self.offset as isize + index;
            if new_offset > 0 && new_offset <= self.memory_length as isize {
                self.offset = new_offset as usize;
                Some(())
            } else {
                None
            }
        }

        /// Changes the memory length, if the new length is valid.
        /// 
        /// # Safety
        /// 
        /// This function is unsafe because it directly modifies the memory length. Ensure that the new length is 
        /// valid and that the memory block can accommodate the new length.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// unsafe {
        ///     assert!(ptr.change_memory_length(10).is_some());
        /// }
        /// ```
        pub unsafe fn change_memory_length(&mut self, memory_length: usize) -> Option<()> {
            if memory_length <= 0 || self.offset > memory_length || self.offset < memory_length {
                return None;
            }

            self.memory_length = memory_length;
            Some(())
        }

        /// Releases the pointer and returns the value it points to, if valid.
        /// 
        /// This method takes ownership of the pointer and returns the value it points to, ensuring that 
        /// the pointer is valid and properly aligned.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let value = ptr.release_ptr().unwrap();
        /// ```
        pub fn release_ptr(self) -> Option<T> {
            if self.check_ptr() {
                unsafe {
                    let ptr: T = *self.ptr;
                    drop(self);
                    Some(ptr)
                }
            } else {
                None
            }
        }

        /// Sets the pointer to null and resets the memory length and offset.
        /// 
        /// This method is useful for invalidating a pointer and ensuring that it no longer points to any memory.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// ptr.set_null();
        /// ```
        #[inline]
        pub fn set_null(&mut self) -> () {
            if self.check_ptr() {
                self.memory_length = 0;
                self.offset = 0;
                self.ptr = std::ptr::null();
            }
        }

        /// Returns the memory address of the pointer as a hexadecimal string.
        /// 
        /// This method is useful for debugging and logging purposes to inspect the raw memory address.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let address = ptr.memory_address();
        /// ```
        #[inline]
        pub fn memory_address(&self) -> String {
            format!("{:x}", self.ptr as usize)
        }

        /// Converts the `ConstRawPtr` to a mutable pointer.
        /// 
        /// This method creates a mutable version of the `ConstRawPtr`, which allows for modification of the 
        /// underlying data.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let mut_ptr = ptr.as_mut();
        /// ```
        #[inline]
        pub fn as_mut(&self) -> super::mut_raw_ptr::MutRawPtr<T> {
            super::mut_raw_ptr::MutRawPtr::new(self.ptr as *mut T, self.memory_length, self.offset)
        }

        /// Unwraps the pointer and returns the value it points to, if valid.
        /// 
        /// This method returns the value that the pointer points to, ensuring that the pointer is valid and 
        /// properly aligned.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let value = ptr.unwrap().unwrap();
        /// ```
        pub fn unwrap(self) -> Option<T> {
            if self.check_ptr() {
                Some( unsafe { *self.ptr } )
            } else {
                None
            }
        }

        /// Returns a reference to the value the pointer points to, if valid.
        /// 
        /// This method provides a reference to the value that the pointer points to, ensuring that the pointer 
        /// is valid and properly aligned.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let reference = ptr.ref_const().unwrap();
        /// ```
        pub fn ref_const(&self) -> Option<&T> {
            if self.check_ptr() {
                Some( unsafe { & *self.ptr } )
            } else {
                None
            }
        }

        /// Checks if the pointer is null.
        /// 
        /// This method determines if the pointer is null, which is useful for validation and error checking.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// assert!(ptr.is_null());
        /// ```
        #[inline]
        pub fn is_null(&self) -> bool {
            self.ptr.is_null()
        }

        /// Returns the size of the type `T`.
        /// 
        /// This method provides the size of the type `T` in bytes, which is useful for memory allocation and 
        /// pointer arithmetic.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let size = ConstRawPtr::<i32>::size_of();
        /// ```
        #[inline]
        pub fn size_of() -> usize {
            std::mem::size_of::<T>()
        }

        /// Casts the pointer to a `ConstRawPtr` of another type `U`.
        /// 
        /// This method allows you to reinterpret the pointer as a different type, ensuring that the new type 
        /// is compatible and properly aligned.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let new_ptr = ptr.cast_ptr::<f64>().unwrap();
        /// ```
        pub fn cast_ptr<U: Sized + Copy + Send + Sync>(&self) -> Option<ConstRawPtr<U>> {
            if !self.ptr.is_null() {
                Some(ConstRawPtr {
                     ptr: self.ptr as *const U, memory_length: self.memory_length, offset: self.offset
                })
            } else {
                None
            }
        }
    }

    impl<T: Sized + Copy + Send + Sync> Clone for ConstRawPtr<T> {
        fn clone(&self) -> Self {
            Self { ptr: self.ptr.clone(), memory_length: self.memory_length, offset: self.offset }
        }
    }

    impl<T: Sized + Copy + Send + Sync> std::fmt::Debug for ConstRawPtr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ConstRawPtr")
                .field("ptr", &self.ptr)
                .field("memory_length", &self.memory_length)
                .field("offset", &self.offset)
                .finish()
        }
    }

    impl<T: Sized + Copy + Send + Sync> PartialEq for ConstRawPtr<T> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr == other.ptr
        }

        fn ne(&self, other: &Self) -> bool {
            self.ptr != other.ptr
        }
    }

    impl<T: Sized + Copy + Send + Sync> Drop for ConstRawPtr<T> {
        fn drop(&mut self) {
            if self.check_ptr() {
                unsafe {
                    let layout: std::alloc::Layout = std::alloc::Layout::new::<T>();
                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
            self.memory_length = 0;
            self.offset = 0;
            self.ptr = std::ptr::null();
        }
    }
}

pub mod mut_raw_ptr {
    pub struct MutRawPtr<T> 
    where  T: Sized + Copy + Send + Sync
    {
        ptr: *mut T,
        memory_length: usize,
        offset: usize,
    }

    impl<T: Sized + Copy + Send + Sync> MutRawPtr<T> {
        /// Creates a new `MutRawPtr` with the given pointer, memory length, and offset.
        /// 
        /// This method ensures that the pointer is properly aligned and that the offset is within the bounds 
        /// of the allocated memory length.
        /// 
        /// # Panics
        /// 
        /// Panics if the pointer is not aligned to `T` or if the offset is not within the bounds of the memory length.
        /// 
        /// # Examples
        /// 
        /// ```rust
        /// let alloc_ptr: *mut i32 = ...; // Assume this is a properly allocated and aligned pointer either from C or using Rust's std::alloc::alloc and std::alloc::Layout otherwise it will panic.
        /// let ptr = MutRawPtr::new(alloc_ptr, 1, 1);
        /// ```
        #[inline]
        pub fn new(ptr: *mut T, memory_length: usize, offset: usize) -> Self {
            assert!((ptr as usize) % std::mem::align_of::<T>() == 0, "box_raw_ptr Err: Memory Not Aligned");
            assert!(offset <= memory_length && offset > 0, "box_raw_ptr Err: Offset Is Not Within Bounds");
            Self { ptr, memory_length, offset, }
        }

        /// Creates a new `MutRawPtr` with a null mutable pointer and zero memory length and offset.
    /// 
    /// This is useful for creating a placeholder `MutRawPtr` that can later be assigned a valid mutable pointer.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let null_ptr = MutRawPtr::<i32>::nullptr();
    /// ```
    #[inline]
    pub fn nullptr() -> Self {
        Self { ptr: std::ptr::null_mut(), memory_length: 0, offset: 0 }
    }

    /// Manually drops the `MutRawPtr` instance.
    /// 
    /// # Safety
    /// 
    /// This function is unsafe because it drops the instance manually, which can lead to undefined behavior 
    /// if the instance is used after being dropped.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// unsafe {
    ///     mut_ptr.manual_drop();
    /// }
    /// ```
    #[inline]
    pub unsafe fn manual_drop(self) -> () {
        drop(self);
    }

    /// Checks if the current offset is within the bounds of the memory length.
    /// 
    /// This method ensures that the mutable pointer is pointing to a valid position within the allocated memory block.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(mut_ptr.check_bounds());
    /// ```
    #[inline]
    pub fn check_bounds(&self) -> bool {
        (1..=self.memory_length).contains(&self.offset)
    }

    /// Checks if the mutable pointer is not null and properly aligned.
    /// 
    /// This method ensures that the mutable pointer is valid and meets the alignment requirements of `T`.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(mut_ptr.check_ptr());
    /// ```
    pub fn check_ptr(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let align: usize = std::mem::align_of::<T>();
        (self.ptr as usize) % align == 0
    }

    /// Returns the current offset.
    /// 
    /// This method provides the current offset within the memory block.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let offset = mut_ptr.check_offset();
    /// ```
    pub fn check_offset(&self) -> usize {
        self.offset
    }

    /// Returns the current memory length.
    /// 
    /// This method provides the total length of the memory block that the mutable pointer points to.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let length = mut_ptr.check_memory_length();
    /// ```
    pub fn check_memory_length(&self) -> usize {
        self.memory_length
    }

    /// Changes the offset by a given index, if the resulting offset is within bounds.
    /// 
    /// This method allows you to move the mutable pointer by a specified index within the memory block, 
    /// ensuring that the new offset is within bounds.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(mut_ptr.change_offset(2).is_some());
    /// ```
    pub fn change_offset(&mut self, index: isize) -> Option<()> {
        if !self.check_ptr() {
            return None;
        }
        let new_offset: isize = self.offset as isize + index;
        if new_offset > 0 && new_offset <= self.memory_length as isize {
            self.offset = new_offset as usize;
            Some(())
        } else {
            None
        }
    }

    /// Changes the memory length, if the new length is valid.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(mut_ptr.change_memory_length(10).is_some());
    /// ```
    pub fn change_memory_length(&mut self, memory_length: usize) -> Option<()> {
        if memory_length <= 0 || self.offset > memory_length {
            return None;
        }

        self.memory_length = memory_length;
        Some(())
    }

    /// Releases the mutable pointer and returns the value it points to, if valid.
    /// 
    /// This method takes ownership of the mutable pointer and returns the value it points to, ensuring that 
    /// the pointer is valid and properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let value = mut_ptr.release_ptr().unwrap();
    /// ```
    pub fn release_ptr(self) -> Option<T> {
        if self.check_ptr() {
            unsafe {
                let ptr: T = *self.ptr;
                drop(self);
                Some(ptr)
            }
        } else {
            None
        }
    }

    /// Sets the mutable pointer to null and resets the memory length and offset.
    /// 
    /// This method is useful for invalidating a mutable pointer and ensuring that it no longer points to any memory.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// mut_ptr.set_null();
    /// ```
    #[inline]
    pub fn set_null(&mut self) -> () {
        if self.check_ptr() {
            self.memory_length = 0;
            self.offset = 0;
            self.ptr = std::ptr::null_mut();
        }
    }

    /// Returns the memory address of the mutable pointer as a hexadecimal string.
    /// 
    /// This method is useful for debugging and logging purposes to inspect the raw memory address.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let address = mut_ptr.memory_address();
    /// ```
    #[inline]
    pub fn memory_address(&self) -> String {
        format!("{:x}", self.ptr as usize)
    }

    /// Converts the `MutRawPtr` to a constant pointer (`ConstRawPtr`).
    /// 
    /// This method creates a constant version of the `MutRawPtr`, which allows for read-only access to the 
    /// underlying data.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let const_ptr = mut_ptr.as_const();
    /// ```
    pub fn as_const(&self) -> super::const_raw_ptr::ConstRawPtr<T> {
        super::const_raw_ptr::ConstRawPtr::new(self.ptr as *const T, self.memory_length, self.offset)
    }

    /// Unwraps the mutable pointer and returns the value it points to, if valid.
    /// 
    /// This method returns the value that the mutable pointer points to, ensuring that the pointer is valid and 
    /// properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let value = mut_ptr.unwrap().unwrap();
    /// ```
    pub fn unwrap(self) -> Option<T> {
        if self.check_ptr() {
            Some( unsafe { *self.ptr } )
        } else {
            None
        }
    }

    /// Returns a reference to the value the mutable pointer points to, if valid.
    /// 
    /// This method provides a reference to the value that the mutable pointer points to, ensuring that the pointer 
    /// is valid and properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let reference = mut_ptr.ref_const().unwrap();
    /// ```
    pub fn ref_const(&self) -> Option<&T> {
        if self.check_ptr() {
            Some( unsafe { & *self.ptr } )
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value the mutable pointer points to, if valid.
    /// 
    /// This method provides a mutable reference to the value that the mutable pointer points to, ensuring that 
    /// the pointer is valid and properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let mut reference = mut_ptr.ref_mut().unwrap();
    /// *reference = 42;
    /// ```
    pub fn ref_mut(&self) -> Option<&mut T> {
        if self.check_ptr() {
            unsafe { Some(&mut *self.ptr) }
        } else {
            None
        }
    }

    /// Checks if the mutable pointer is null.
    /// 
    /// This method determines if the mutable pointer is null, which is useful for validation and error checking.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// assert!(mut_ptr.is_null());
    /// ```
    #[inline]
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }

    /// Returns the size of the type `T`.
    /// 
    /// This method provides the size of the type `T` in bytes, which is useful for memory allocation and 
    /// pointer arithmetic.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let size = mut_ptr.size_of();
    /// ```
    #[inline]
    pub fn size_of(&self) -> usize {
        std::mem::size_of::<T>()
    }

    /// Casts the mutable pointer to a `MutRawPtr` of another type `U`.
    /// 
    /// This method allows you to reinterpret the mutable pointer as a different type, ensuring that the new type 
    /// is compatible and properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// let new_ptr = mut_ptr.cast_ptr::<f64>().unwrap();
    /// ```
    pub fn cast_ptr<U: Sized + Copy + Send + Sync>(&self) -> Option<MutRawPtr<U>> {
        if !self.ptr.is_null() {
            Some(MutRawPtr {
                ptr: self.ptr as *mut U,
                memory_length: self.memory_length,
                offset: self.offset,
            })
        } else {
            None
        }
    }

    /// Writes a value into the memory location pointed to by the mutable pointer.
    /// 
    /// This method writes a value into the memory location pointed to by the mutable pointer, ensuring that 
    /// the pointer is valid and properly aligned.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// mut_ptr.write_ptr(42);
    /// ```
    pub fn write_ptr(&mut self, src: T) -> Option<()> {
        if self.check_ptr() {
            return None;
        }
        unsafe {
            std::ptr::write(self.ptr, src);
        }
        Some(())
        }
    }

    impl<T: Sized + Copy + Send + Sync> Clone for MutRawPtr<T> {
        fn clone(&self) -> Self {
            Self { ptr: self.ptr.clone(), memory_length: self.memory_length, offset: self.offset }
        }
    }

    impl<T: Sized + Copy + Send + Sync> std::fmt::Debug for MutRawPtr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("MutRawPtr")
                .field("ptr", &self.ptr)
                .field("memory_length", &self.memory_length)
                .field("offset", &self.offset)
                .finish()
        }
    }

    impl<T: Sized + Copy + Send + Sync> PartialEq for MutRawPtr<T> {
        fn eq(&self, other: &Self) -> bool {
            self.ptr == other.ptr
        }

        fn ne(&self, other: &Self) -> bool {
            self.ptr != other.ptr
        }
    }

    impl<T: Sized + Copy + Send + Sync> Drop for MutRawPtr<T> {
        fn drop(&mut self) {
            if self.check_ptr() {
                unsafe {
                    let layout: std::alloc::Layout = std::alloc::Layout::new::<T>();
                    std::alloc::dealloc(self.ptr as *mut u8, layout);
                }
            }
            self.memory_length = 0;
            self.offset = 0;
            self.ptr = std::ptr::null_mut();
        }
    }
}

#[cfg(test)]
mod box_raw_ptr_tests {
     use super::{const_safe_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};

    #[test]
    fn c_allocator_test() -> () {
        /* Tests If Allocator Works */
        let alloc: *mut i32 = unsafe { std::alloc::alloc(std::alloc::Layout::new::<i32>()) as *mut i32 };
        let _ = MutRawPtr::new(alloc, 1, 1);
    }

    #[test]
    fn it_works() -> () {
        // Allocate properly aligned memory for an i32
        let alloc: *const i32 = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(20, 4).unwrap()) as *const i32 };
        let mut ptr: ConstRawPtr<i32> = ConstRawPtr::new(alloc, 5, 1);
        ptr.change_offset(4).unwrap();
    }
}

