//! # box_raw_ptr
//!
//! `box_raw_ptr` is a Rust library providing safe wrappers for working with raw pointers,
//! specifically `*const T` and `*mut T`. These wrappers ensure memory safety by encapsulating
//! the raw pointers in safe abstractions and providing safe methods for working with them.
//!
//! ## Features
//!
//! - **ConstRawPtr**: A wrapper for `*const T` providing methods for safely working with constant raw pointers.
//! - **MutRawPtr**: A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.
//!
//! ## Example
//!
//! ```rust
//! use box_raw_ptr::const_raw_ptr::ConstRawPtr;
//!
//! fn main() {
//!     let ptr = Box::new(42);
//!     let raw_ptr = ConstRawPtr::new_const_ptr(&*ptr);
//!
//!     // Print the memory address of the raw pointer
//!     raw_ptr.print_const_ptr(true);
//!
//!     // Print the value pointed to by the raw pointer
//!     raw_ptr.print_const_ptr(false);
//! }
//! ```
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! box_raw_ptr = "0.1.0"
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
//!
//! ## Contributions
//!
//! Contributions are welcome! Please feel free to submit a pull request or open an issue
//! on the [GitHub repository](https://github.com/yourusername/yourrepository).
//!
//! ```
//!
//! extern crate box_raw_ptr;
//!
//! use box_raw_ptr::const_raw_ptr::ConstRawPtr;
//!
//! fn main() {
//!     let ptr = Box::new(42);
//!     let raw_ptr = ConstRawPtr::new_const_ptr(&*ptr);
//!
//!     // Print the memory address of the raw pointer
//!     raw_ptr.print_const_ptr(true);
//!
//!     // Print the value pointed to by the raw pointer
//!     raw_ptr.print_const_ptr(false);
//! }
//! ```


pub mod box_raw_ptr {
    pub mod const_raw_ptr {
        /// A wrapper for `*const T` providing methods for safely working with constant raw pointers.
        pub struct ConstRawPtr<T> 
        where T: std::fmt::Debug, T: std::marker::Send, T: std::marker::Sync, T: core::marker::Copy
        {
            const_ptr: Box<*const T>,
        }

        impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> ConstRawPtr<T> {
            /// Constructs a new `ConstRawPtr<T>` instance with the given constant raw pointer.
            pub fn new_const_ptr(ptr: *const T) -> Self {
                Self { const_ptr: Box::new(ptr) }
            }
            
            /// Constructs a new `ConstRawPtr<T>` instance with a null constant raw pointer.
            pub fn const_null_ptr() -> Self {
                Self { const_ptr: Box::new(std::ptr::null()) }
            }
            
            /// Returns the raw pointer if it is not null, wrapped in an `Option`. Returns `None` otherwise.
            pub fn unbox_const(&self) -> Option<*const T> {
                if !self.const_ptr.is_null() {
                    Some(*self.const_ptr)
                } else {
                    None
                }
            }

            /// Returns the underlying value if it is not null, wrapped in an `Option`. Returns `None` otherwise.
            pub fn unwrap_const(self) -> Option<T> {
                if !self.const_ptr.is_null() {
                    Some( unsafe { **self.const_ptr } )
                } else {
                    None
                }
            }

            /// Returns a reference to the value pointed to by the constant raw pointer, wrapped in an `Option`.
            /// Returns `None` if the pointer is null.
            pub fn ref_const(&self) -> Option<&T> {
                if !self.const_ptr.is_null() {
                    Some( unsafe { & **self.const_ptr } )
                } else {
                    None
                }
            }

            /// Returns `true` if the constant raw pointer is null, `false` otherwise.
            pub fn is_null(&self) -> bool {
                if !self.const_ptr.is_null() {
                    true
                } else {
                    false
                }
            }

            /// Prints the memory address of the constant raw pointer if `fmt` is true.
            /// Otherwise, prints the value pointed to by the constant raw pointer if it's not null.
            /// Returns a cloned instance of `ConstRawPtr<T>`.
            pub fn print_const_ptr(&self, fmt: bool) -> Self {
                if !self.const_ptr.is_null() {
                    if fmt {
                        println!("{:?}", *self.const_ptr);
                    } else {
                        unsafe { println!("{:?}", **self.const_ptr); }
                    }
                }

                Self { const_ptr: Box::new(*self.const_ptr.clone()) }
            }
        }

        // Clone implementation for ConstRawPtr<T>
        impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> Clone for ConstRawPtr<T> {
            fn clone(&self) -> Self {
                Self {
                    const_ptr: self.const_ptr.clone(),
                }
            }
        }
    }

    pub mod mut_raw_ptr {
        /// A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.
        pub struct MutRawPtr<T>
        where T: std::fmt::Debug, T: std::marker::Send, T: std::marker::Sync, T: core::marker::Copy
        {
            mut_ptr: Box<*mut T>,
        }

        impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> MutRawPtr<T> {
            /// Constructs a new `MutRawPtr<T>` instance with the given mutable raw pointer.
            pub fn new_mut_ptr(ptr: *mut T) -> Self {
                Self { mut_ptr: Box::new(ptr) }
            }
        
            /// Constructs a new `MutRawPtr<T>` instance with a null mutable raw pointer.
            pub fn mut_null_ptr() -> Self {
                Self { mut_ptr: (Box::new(std::ptr::null_mut())) }
            }
        
            /// Returns the raw pointer if it is not null, wrapped in an `Option`. Returns `None` otherwise.
            pub fn unbox_mut(self) -> Option<*mut T> {
                if !self.mut_ptr.is_null() {
                    Some(*self.mut_ptr)
                } else {
                    None
                }
            }

            /// Returns the underlying value if it is not null, wrapped in an `Option`. Returns `None` otherwise.
            pub fn unwrap_mut(self) -> Option<T> {
                if !self.mut_ptr.is_null() {
                    Some( unsafe { **self.mut_ptr } )
                } else {
                    None
                }
            }

            /// Returns a reference to the value pointed to by the mutable raw pointer, wrapped in an `Option`.
            /// Returns `None` if the pointer is null.
            pub fn ref_mut(&self) -> Option<&T> {
                if !self.mut_ptr.is_null() {
                    Some( unsafe { & **self.mut_ptr } )
                } else {
                    None
                }
            }

            /// Returns a mutable reference to the value pointed to by the mutable raw pointer, wrapped in an `Option`.
            /// Returns `None` if the pointer is null.
            pub fn mutref_mut(&self) -> Option<&mut T> {
                if !self.mut_ptr.is_null() {
                    Some( unsafe { &mut **self.mut_ptr } )
                } else {
                    None
                }
            }

            /// Returns `true` if the mutable raw pointer is null, `false` otherwise.
            pub fn is_null(&self) -> bool {
                if !self.mut_ptr.is_null() {
                    true
                } else {
                    false
                }
            }

            /// Prints the memory address of the mutable raw pointer if `fmt` is true.
            /// Otherwise, prints the value pointed to by the mutable raw pointer if it's not null.
            /// Returns a cloned instance of `MutRawPtr<T>`.
            pub fn print_mut_ptr(&self, fmt: bool) -> Self {
                if !self.mut_ptr.is_null() {
                    if fmt {
                        println!("{:?}", *self.mut_ptr);
                    } else {
                        unsafe { println!("{:?}", **self.mut_ptr); }
                    }
                }

                Self { mut_ptr: Box::new(*self.mut_ptr.clone()) }
            }

            /// Writes the value `src` to the memory location pointed to by the mutable raw pointer.
            /// Returns a new instance of `MutRawPtr<T>` with the same raw pointer after the write operation.
            pub fn write_mut_ptr(&mut self, src: T) -> Option<Self> {
                if !self.mut_ptr.is_null() {
                    unsafe {
                        std::ptr::write(*self.mut_ptr, src);
                        Some( Self { mut_ptr: Box::new(*self.mut_ptr) } )
                    }
                } else {
                    None
                }
            }
        }

        // Clone implementation for MutRawPtr<T>
        impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> Clone for MutRawPtr<T> {
            fn clone(&self) -> Self {
                Self {
                    mut_ptr: self.mut_ptr.clone(),
                }
            }
        }
    }
}