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
//! use box_raw_ptr::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};
//!
//! fn main() {
//!     // External C++ Pointer Function Example:
//!     extern "C" {
//!         fn cpp_ptr() -> *mut i32;
//!     }
//!
//!     // Get Unsafe External C++ Pointer
//!     let cpp_ptr: *mut i32 = unsafe { cpp_ptr() };
//!     
//!     // Convert Unsafe External C++ Pointer To MutRawPtr Of Type i32
//!     let mut mut_safe_ptr: MutRawPtr<i32> = MutRawPtr::new_mut_ptr(cpp_ptr);
//!
//!     // Write To The Safe Pointer
//!     mut_safe_ptr.write_mut_ptr(20 as i32);
//!
//!     // Print Value Of mut_safe_ptr Note: Uses .unwrap() as ptr is guaranteed not to be NULL
//!     println!("{}", t.unwrap_mut().unwrap());
//!
//!     // Writing To MutRawPtr<T> Example:
//!     let mut_ptr: MutRawPtr<u8> = MutRawPtr::new_mut_ptr(&mut 13_u8 as *mut u8);
//!
//!     // Cast MutRawPtr<T> To Option<*mut U>
//!     let u_ptr: *mut i32 = mut_ptr.mut_cast_ptr::<i32>().unwrap();
//!
//!     match MutRawPtr::new_mut_ptr(u_ptr).write_mut_ptr(20 as i32) {
//!         Some(ptr) => {
//!             // Print MutRawPtr Memory Address
//!             println!("{}", ptr.mut_mem_addr());
//!         }
//!         None => (),
//!     }
//!
//!     // Pointer Arithmetic For A [T; U] That Returns The Index Value In The Array Example:
//!     let arr: [i32; 5] = [1,2,3,4,5];
//!
//!     // Create New ConstRawPtr<i32> From The Array As A Pointer
//!     let arr_ptr: ConstRawPtr<i32> = ConstRawPtr::new_const_ptr(arr.as_ptr());
//!
//!     // Set The Index Of The Pointer
//!     ConstRawPtr::set_idx_ptr(&arr_ptr, 2)
//!         .inspect(|x| {
//!             let t: i32 = x.clone().unwrap_const().unwrap();
//!             // 2 Indexed From arr Equals 3
//!             assert_eq!(3, t);
//!         });
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
    pub struct ConstRawPtr<
        T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy
    >(Box<*const T>);

    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> ConstRawPtr<T> {
        /// Constructs a new `ConstRawPtr<T>` instance with the given constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// ```   
        #[inline]
        pub fn new_const_ptr(ptr: *const T) -> Self {
            Self(Box::new(ptr))
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
            Self(Box::new(std::ptr::null()))
        }

        /// Drops the value pointed to by the constant raw pointer.
        ///
        /// # Safety
        /// 
        /// This function is inherently unsafe because it manually drops the value pointed to by the constant raw pointer.
        /// Calling this method twice on the same value can lead to undefined behavior.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// unsafe { const_ptr.const_manual_drop() };
        /// ```
        pub unsafe fn const_manual_drop(self) -> () {
            let _ = Box::from_raw(*self.0 as *mut T);
        }

        /// Releases the constant raw pointer and returns it.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let mut const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// let released_ptr = const_ptr.const_release_ptr();
        /// ```
        pub fn const_release_ptr(&mut self) -> *const T {
            let return_ptr: *const T = *self.0;
            *self.0 = std::ptr::null();
            return_ptr
        }

        /// Sets the constant raw pointer to null.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let mut const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// const_ptr.const_set_null();
        /// ```
        pub fn const_set_null(&mut self) -> () {
            *self.0 = std::ptr::null();
        }

        /// Returns the memory address of the constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// let mem_addr = const_ptr.const_mem_addr();
        /// ```
        pub fn const_mem_addr(&self) -> String {
            format!("{:x}", *self.0 as usize)
        }

        /// Returns a mutable reference to the value pointed to by the constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&mut value as *const i32);
        /// let mut_ref = unsafe { &mut *const_ptr.as_mut() };
        /// ```
        pub fn as_mut(&self) -> super::mut_raw_ptr::MutRawPtr<T> {
            super::mut_raw_ptr::MutRawPtr::new_mut_ptr(*self.0 as *mut T)
        }
        
        /// Returns the raw pointer
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
        ///
        /// assert_eq!(const_ptr.unbox_const(), &ptr_value as *const i32);
        /// ```      
        pub fn unbox_const(&self) -> *const T {
            *self.0
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
            if !self.0.is_null() {
                Some( unsafe { **self.0 } )
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
            if !self.0.is_null() {
                Some( unsafe { & **self.0} )
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
            self.0.is_null()
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
            if !self.0.is_null() {
                Some( *self.0 as *const U )
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
            if !self.0.is_null() {
                Some( unsafe {
                    Self(Box::new(self.0.add(idx)))
                })
            } else {
                None
            }
        }
    }

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
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> Clone for ConstRawPtr<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    /// Implements the `Debug` trait for `ConstRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
    ///
    /// let ptr_value: i32 = 42;
    /// let const_ptr = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
    /// println!("{:?}", const_ptr);
    /// ```
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> std::fmt::Debug for ConstRawPtr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("ConstRawPtr").field(&self.0).finish()
        }
    }

    /// Implements the `PartialEq` trait for `ConstRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::const_raw_ptr::ConstRawPtr;
    ///
    /// let ptr_value: i32 = 42;
    /// let const_ptr1 = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
    /// let const_ptr2 = ConstRawPtr::new_const_ptr(&ptr_value as *const i32);
    ///
    /// assert_eq!(const_ptr1, const_ptr2);
    /// ```
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> PartialEq for ConstRawPtr<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn ne(&self, other: &Self) -> bool {
            self.0 != other.0
        }
    }
}

pub mod mut_raw_ptr {
    /// A wrapper for `*mut T` providing methods for safely working with mutable raw pointers.
    pub struct MutRawPtr<
        T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy
    >(Box<*mut T>);

    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> MutRawPtr<T> {
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
            Self(Box::new(ptr))
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
            Self(Box::new(std::ptr::null_mut()))
        }

        /// Manually drops the `MutRawPtr<T>` instance.
        ///
        /// # Safety
        ///
        /// - This function is inherently unsafe due to dropping the `MutRawPtr<T>` instance.
        /// - Ensure that dropping the instance is appropriate and does not lead to use-after-free or memory leaks.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// unsafe { ptr.mut_manual_drop() };
        /// ```
        pub unsafe fn mut_manual_drop(self) -> () {
            let _ = Box::from_raw(*self.0);
        }

        /// Releases the mutable raw pointer and returns a pointer to the underlying value.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// let released_ptr = ptr.mut_release_ptr();
        /// ```
        pub fn mut_release_ptr(&mut self) -> *mut T {
            let return_ptr: *mut T = *self.0;
            *self.0 = std::ptr::null_mut();
            return_ptr
        }

        /// Sets the mutable raw pointer to null.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// ptr.mut_set_null();
        /// ```
        pub fn mut_set_null(&mut self) -> () {
            *self.0 = std::ptr::null_mut();
        }

        /// Returns the memory address of the mutable raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::const_raw_ptr::MutRawPtr;
        ///
        /// let ptr_value: i32 = 42;
        /// let const_ptr = MutRawPtr::new_const_ptr(&ptr_value as *const i32);
        /// let mem_addr = const_ptr.const_mem_addr();
        /// ```
        pub fn mut_mem_addr(&self) -> String {
            format!("{:x}", *self.0 as usize)
        }

        /// Casts the mutable raw pointer to a constant raw pointer.
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        /// let const_ptr = ptr.as_const();
        /// ```
        pub fn as_const(&self) -> super::const_raw_ptr::ConstRawPtr<T> {
            super::const_raw_ptr::ConstRawPtr::new_const_ptr(*self.0 as *const T)
        }
    
        /// Returns the raw pointer
        ///
        /// # Example
        ///
        /// ```
        /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
        ///
        /// let mut value: i32 = 42;
        /// let mut_ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
        ///
        /// assert_eq!(mut_ptr.unbox_mut(), &mut value as *mut i32);
        /// ```
        pub fn unbox_mut(self) -> *mut T {
            *self.0
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
            if !self.0.is_null() {
                Some( unsafe { **self.0 } )
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
            if !self.0.is_null() {
                Some( unsafe { & **self.0 } )
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
            if !self.0.is_null() {
                Some( unsafe { &mut **self.0 } )
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
            self.0.is_null()
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
            if !self.0.is_null() {
                Some( *self.0 as *mut U )
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
            if !self.0.is_null() {
                Some( unsafe {
                    Self(Box::new(self.0.add(idx)))
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
            if !self.0.is_null() {
                unsafe {
                    std::ptr::write(*self.0, src);
                    Some(Self(Box::new(*self.0)))
                }
            } else {
                None
            }
        }   
    }

    /// Implements the `Clone` trait for `MutRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
    ///
    /// let mut value: i32 = 42;
    /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
    /// let cloned_ptr = ptr.clone();
    /// ```
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> Clone for MutRawPtr<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    /// Implements the `Debug` trait for `MutRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
    ///
    /// let mut value: i32 = 42;
    /// let mut ptr = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
    /// println!("{:?}", ptr);
    /// ```
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> std::fmt::Debug for MutRawPtr<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("MutRawPtr").field(&self.0).finish()
        }
    }

    /// Implements the `PartialEq` trait for `MutRawPtr<T>`.
    ///
    /// # Example
    ///
    /// ```
    /// use box_raw_ptr::mut_raw_ptr::MutRawPtr;
    ///
    /// let mut value: i32 = 42;
    /// let mut ptr1 = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
    /// let mut ptr2 = MutRawPtr::new_mut_ptr(&mut value as *mut i32);
    ///
    /// assert_eq!(ptr1, ptr2);
    /// ```
    impl<T: std::fmt::Debug + std::marker::Send + std::marker::Sync + std::marker::Copy> PartialEq for MutRawPtr<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn ne(&self, other: &Self) -> bool {
            self.0 != other.0
        }
    }
}

#[cfg(test)]
mod box_raw_ptr_tests {
    #[allow(unused_imports)]
    use super::{const_raw_ptr::ConstRawPtr, mut_raw_ptr::MutRawPtr};

    #[test]
    fn test_ptr() -> () {}
}