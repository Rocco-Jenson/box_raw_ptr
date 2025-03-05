/*
*       ____             ____                 ____  __      
*      / __ )____  _  __/ __ \____ __      __/ __ \/ /______
*     / __  / __ \| |/_/ /_/ / __ `/ | /| / / /_/ / __/ ___/
*    / /_/ / /_/ />  </ _, _/ /_/ /| |/ |/ / ____/ /_/ /    
*   /_____/\____/_/|_/_/ |_|\__,_/ |__/|__/_/    \__/_/     
*                                               
*   
*   Copyright (c) 2024 Rocco Zinedine Samuel Jenson
*   
*   Licensed under the MIT License (the "License");
*   you may not use this file except in compliance with the License.
*   You may obtain a copy of the License at
*
*   https://opensource.org/licenses/MIT
*   
*   Unless required by applicable law or agreed to in writing, software
*   distributed under the License is distributed on an "AS IS" BASIS,
*   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
*   See the License for the specific language governing permissions and
*   limitations under the License.
*/

use box_raw_ptr::mut_raw_ptr::MutRawPtr;
/* 
Import C file
NOTE: Correctly specify type of file (ex: kind = "dylib" || "static") or linker will throw error 
*/
#[link(name = "example", kind = "static")]
extern "C" {
    fn get_c_ptr() -> *mut std::ffi::c_int;
}

fn main() {
    /* Get int* from C and convert to MutRawPtr<i32> */
    let ptr: *mut i32 = unsafe {
        get_c_ptr()
    };

    let mut safeptr: MutRawPtr<i32> = MutRawPtr::new(ptr, 1, 0);

    /* Write 100 to safeptr */
    assert!(safeptr.write_ptr(100 as i32).is_some());

    /* Print memory address of C pointer and the underlying value */
    println!("{} -> {}", safeptr.memory_address(), safeptr.access().unwrap());
    
    /* 
    Memory is automatically dropped using free() from the box_raw_ptr Global Allocator 
    See allocator.rs and allocator.c for implementation 
    */
}
