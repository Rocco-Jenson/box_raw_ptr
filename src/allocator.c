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

/* Wrapper around malloc() and free() to remove libc dependency */
#include <stdlib.h>
#include <stdint.h>

#define X86_X64_SIZE 0xFFFFFFFFFFFFFFFFu /* Max 64 bit integer */
#define X86_SIZE  0xFFFFFFFF /* Max 32 bit integer */

/* Check if architecture is x86_x64 */
#if UINTPTR_MAX == X86_X64_SIZE
    typedef uint64_t arch_type;
/* Check if architecture is x86 */
#elif UINTPTR_MAX == X86_SIZE
    typedef uint32_t arch_type;
#else
    #error "Unsupported architecture: Only 64 and 32 bit architectures supported"
#endif

void* c_global_allocator(arch_type bytes) {
    /* Return the pointer even if it's NULL to let Rust handle allocation errors */
    return malloc(bytes);
}

void c_global_deallocator(void* ptr) {
    /* Rust allocator manages NULL check */
    free(ptr);
}
