/* Wrapper around malloc() and free() to remove libc dependency */
#include <stdint.h>
#include <stdlib.h>

/* Check if architecture is x64 or ARM
#if defined(__x86_64__) || defined(_M_X64) || defined(__aarch64)
    typedef uint64_t arch_type;
/* Check if architecture is x86 or ARM x86 */
#elif defined(__i386__) || defined(_M_IX86) || defined(__arm__) || defined(_M_ARM)
    typedef uint32_t arch_type;
#else
    #error "Unsupported architecture: Only Intel (x86/x64) and ARM (32-bit/64-bit) architectures are supported"
#endif

volatile static uint8_t HEAP_COUNT = 0;

void* c_global_allocator(arch_type bytes) {   
    void* ptr = malloc(bytes);
    if (ptr != NULL) {
        HEAP_COUNT++;
    }
    /* Return the pointer even if it's NULL to let Rust handle allocation errors */
    return ptr;
}

void c_global_deallocator(void* ptr) {
    if (ptr != NULL) {
        HEAP_COUNT--;
        free(ptr);
    }
}

arch_type global_allocation_info() {
    return HEAP_COUNT;
}
