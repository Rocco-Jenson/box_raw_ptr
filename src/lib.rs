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
//!     let raw_ptr = ConstRawPtr::new_const_ptr(1 as *const i32);
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
//! box_raw_ptr = "0.2.4"
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.
//!
//! ## Contributions
//!
//! Contributions are welcome! Please feel free to submit a pull request or open an issue
//! on the [GitHub repository](https://github.com/Rocco-Jenson/box_raw_ptr).
//!
//! ```

pub mod const_raw_ptr {
    /// A wrapper for `*const T` providing methods for safely working with constant raw pointers.
    pub struct ConstRawPtr<T> 
    where T: std::fmt::Debug, T: std::marker::Send, T: std::marker::Sync, T: core::marker::Copy
    {
        const_ptr: Box<*const T>,
    }

    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> ConstRawPtr<T> {
        /// Constructs a new `ConstRawPtr<T>` instance with the given constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// ```  
        
        #[inline]
        pub fn new_const_ptr(ptr: *const T) -> Self {
            Self { const_ptr: Box::new(ptr) }
        }
        
        /// Constructs a new `ConstRawPtr<T>` instance with a null constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let null_ptr = ConstRawPtr::const_null_ptr();
        /// ```
        
        #[inline]
        pub fn const_null_ptr() -> Self {
            Self { const_ptr: Box::new(std::ptr::null()) }
        }
        
        /// Returns the raw pointer if it is not null, wrapped in an `Option`. Returns `None` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        ///
        /// assert_eq!(const_ptr.unbox_const(), Some(&ptr_value as *const i32));
        /// ```
        
        pub fn unbox_const(&self) -> Option<*const T> {
            if !self.const_ptr.is_null() {
                Some(*self.const_ptr)
            } else {
                None
            }
        }

        /// Returns the underlying value if it is not null, wrapped in an `Option`. Returns `None` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        ///
        /// assert_eq!(const_ptr.unwrap_const(), Some(ptr_value));
        /// ```
        
        pub fn unwrap_const(self) -> Option<T> {
            if !self.const_ptr.is_null() {
                Some( unsafe { **self.const_ptr } )
            } else {
                None
            }
        }

        /// Returns a reference to the value pointed to by the constant raw pointer, wrapped in an `Option`.
        /// Returns `None` if the pointer is null.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        ///
        /// assert_eq!(const_ptr.ref_const(), Some(&ptr_value));
        /// ```
        
        pub fn ref_const(&self) -> Option<&T> {
            if !self.const_ptr.is_null() {
                Some( unsafe { & **self.const_ptr} )
            } else {
                None
            }
        }

        /// Returns `true` if the constant raw pointer is null, `false` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let null_ptr = ConstRawPtr::<i32>::const_null_ptr();
        /// assert!(null_ptr.is_null());
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// assert!(!const_ptr.is_null());
        /// ```
        
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
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// const_ptr.print_const_ptr(true); // prints the memory address
        /// const_ptr.print_const_ptr(false); // prints the value pointed to by the pointer
        /// ```
        
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

        /// Casts the constant raw pointer to a pointer of a different type `U`.
        /// 
        /// # Safety
        /// 
        /// - This function is inherently unsafe due to casting a raw pointer.
        /// - Ensure the cast preserves memory safety and alignment requirements.
        /// - Null-pointer checking is performed to mitigate unsafe behavior.
        /// 
        /// # Returns
        /// 
        /// An `Option` containing the casted pointer of type `*const U`.
        /// 
        /// # Example
        /// 
        /// ```
        /// # use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        /// # let ptr_value: i32 = 42;
        /// # let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// let casted_ptr = const_ptr.const_cast_ptr::<u8>();
        /// assert_eq!(casted_ptr, Some(&ptr_value as *const i32 as *const u8));
        /// ```
        
        pub fn const_cast_ptr<U>(&self) -> Option<*const U> {
            if !self.const_ptr.is_null() {
                Some( *self.const_ptr as *const U )
            } else {
                None
            }
        }

        /// Returns a new instance of `ConstRawPtr<T>` pointing to the memory location
        /// obtained by adding the index `idx` to the original pointer, if it is not null.
        ///
        /// # Safety
        ///
        /// - This function is inherently unsafe due to performing pointer arithmetic and constructing a new `ConstRawPtr`.
        /// - It's the caller's responsibility to ensure that the resulting pointer remains within the bounds of valid memory.
        /// - Null-pointer checking is performed to mitigate unsafe behavior.
        ///
        /// # Arguments
        ///
        /// * `idx` - The index to be added to the original pointer.
        ///
        /// # Returns
        ///
        /// An `Option` containing the new `ConstRawPtr` instance pointing to the memory location obtained by adding the index.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let array: [i32; 3] = [1, 2, 3];
        /// let const_ptr = ConstRawPtr::new_const_ptr(&array[0] as *const i32);
        ///
        /// // Get a pointer to the second element of the array
        /// let second_elem_ptr = const_ptr.get_idx_ptr(1);
        ///
        /// assert_eq!(second_elem_ptr.unwrap().unwrap_const(), &array[1]);
        /// ```

        pub fn set_idx_ptr(&self, idx: usize) -> Option<Self> {
            if !self.const_ptr.is_null() {
                Some( unsafe {
                    Self { const_ptr: Box::new(self.const_ptr.add(idx)) }
                })
            } else {
                None
            }
        }
    }

    // Clone implementation for ConstRawPtr<T>
    /// Implements the `Clone` trait for `ConstRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
    ///
    /// let ptr_value: i32 = 42;
    /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
    /// let cloned_ptr = const_ptr.clone();
    /// ```
    
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
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// ```
        
        #[inline]
        pub fn new_mut_ptr(ptr: *mut T) -> Self {
            Self { mut_ptr: Box::new(ptr) }
        }
    
        /// Constructs a new `MutRawPtr<T>` instance with a null mutable raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let null_ptr = MutRawPtr::mut_null_ptr();
        /// ```
        
        #[inline]
        pub fn mut_null_ptr() -> Self {
            Self { mut_ptr: (Box::new(std::ptr::null_mut())) }
        }
    
        /// Returns the raw pointer if it is not null, wrapped in an `Option`. Returns `None` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        ///
        /// assert_eq!(mut_ptr.unbox_mut(), Some(&mut value as *mut i32));
        /// ```
        
        pub fn unbox_mut(self) -> Option<*mut T> {
            if !self.mut_ptr.is_null() {
                Some(*self.mut_ptr)
            } else {
                None
            }
        }

        /// Returns the underlying value if it is not null, wrapped in an `Option`. Returns `None` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        ///
        /// assert_eq!(mut_ptr.unwrap_mut(), Some(value));
        /// ```
        
        pub fn unwrap_mut(self) -> Option<T> {
            if !self.mut_ptr.is_null() {
                Some( unsafe { **self.mut_ptr } )
            } else {
                None
            }
        }

        /// Returns the underlying value if it is not null, wrapped in an `Option`. Returns `None` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        ///
        /// assert_eq!(mut_ptr.unwrap_mut(), Some(value));
        /// ```
        
        pub fn ref_mut(&self) -> Option<&T> {
            if !self.mut_ptr.is_null() {
                Some( unsafe { & **self.mut_ptr } )
            } else {
                None
            }
        }

        /// Returns a mutable reference to the value pointed to by the mutable raw pointer, wrapped in an `Option`.
        /// Returns `None` if the pointer is null.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        ///
        /// assert_eq!(mut_ptr.mutref_mut(), Some(&mut value));
        /// ```
        
        pub fn mutref_mut(&self) -> Option<&mut T> {
            if !self.mut_ptr.is_null() {
                Some( unsafe { &mut **self.mut_ptr } )
            } else {
                None
            }
        }

        /// Returns `true` if the mutable raw pointer is null, `false` otherwise.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let null_ptr = MutRawPtr::<i32>::mut_null_ptr();
        /// assert!(null_ptr.is_null());
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// assert!(!mut_ptr.is_null());
        /// ```
       
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
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// mut_ptr.print_mut_ptr(true); // prints the memory address
        /// mut_ptr.print_mut_ptr(false); // prints the value pointed to by the pointer
        /// ```
        
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

        /// Casts the mutable raw pointer to a pointer of a different type `U`.
        /// 
        /// # Safety
        /// 
        /// - This function is inherently unsafe due to casting a raw pointer.
        /// - Ensure the cast preserves memory safety and alignment requirements.
        /// - Null-pointer checking is performed to mitigate unsafe behavior.
        /// 
        /// # Returns
        /// 
        /// An `Option` containing the casted pointer of type `*mut U`.
        /// 
        /// # Example
        /// 
        /// ```
        /// # use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        /// # let mut value: i32 = 42;
        /// # let mut mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// let casted_ptr = mut_ptr.mut_cast_ptr::<u8>();
        /// assert_eq!(casted_ptr, Some(&mut value as *mut i32 as *mut u8));
        /// ```
        
        pub fn mut_cast_ptr<U>(&self) -> Option<*mut U> {
            if !self.mut_ptr.is_null() {
                Some( *self.mut_ptr as *mut U )
            } else {
                None
            }
        }

        /// Returns a new instance of `MutRawPtr<T>` pointing to the memory location
        /// obtained by adding the index `idx` to the original pointer, if it is not null.
        ///
        /// # Safety
        ///
        /// - This function is inherently unsafe due to performing pointer arithmetic and constructing a new `ConstRawPtr`.
        /// - It's the caller's responsibility to ensure that the resulting pointer remains within the bounds of valid memory.
        /// - Null-pointer checking is performed to mitigate unsafe behavior.
        ///
        /// # Arguments
        ///
        /// * `idx` - The index to be added to the original pointer.
        ///
        /// # Returns
        ///
        /// An `Option` containing the new `MutRawPtr` instance pointing to the memory location obtained by adding the index.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let array: [i32; 3] = [1, 2, 3];
        /// let mut_ptr = MutRawPtr::new_const_ptr(&array[0] as *const i32);
        ///
        /// // Get a pointer to the second element of the array
        /// let second_elem_ptr = mut_ptr.get_idx_ptr(1);
        ///
        /// assert_eq!(second_elem_ptr.unwrap().unwrap_const(), &array[1]);
        /// ```

        pub fn set_idx_ptr(&self, idx: usize) -> Option<Self> {
            if !self.mut_ptr.is_null() {
                Some( unsafe {
                    Self { mut_ptr: Box::new(self.mut_ptr.add(idx)) }
                })
            } else {
                None
            }
        }

        /// Writes the value `src` to the memory location pointed to by the mutable raw pointer.
        /// Returns a new instance of `MutRawPtr<*mut T>` with the same raw pointer after the write operation.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// mut_ptr.write_mut_ptr(24); // writes 24 to the memory location
        /// ```
        
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

    /// Clone implementation for MutRawPtr<T>
    /// Implements the `Clone` trait for `MutRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
    ///
    /// let mut value: i32 = 42;
    /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
    /// let cloned_ptr = mut_ptr.clone();
    /// ```
    
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + core::marker::Copy> Clone for MutRawPtr<T> {
        fn clone(&self) -> Self {
            Self {
                mut_ptr: self.mut_ptr.clone(),
            }
        }
    }
}