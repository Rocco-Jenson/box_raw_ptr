/* Wrapper around malloc() and free() to remove libc dependency */
#include <stdint.h>
#include <stdlib.h>

/* Check if architecture is x64 or ARM x64 */
#if defined(__x86_64__) 
|| defined(_M_X64) 
|| defined(__aarch64)
    typedef uint64_t arch_type;
/* Check if architecture is x86 or ARM x86 */
#elif defined(__i386__)
|| defined(_M_IX86) 
|| defined(__arm__) 
|| defined(_M_ARM)
    typedef uint32_t arch_type;
#else
    #error "Unsupported architecture:
            Only Intel (x86/x64) and ARM (32-bit/64-bit) 
            architectures are supported"
#endif

void* c_global_allocator(arch_type bytes) {
    return malloc(bytes);
}

arch_type c_global_deallocator(void* ptr) {
    free(ptr);
}
