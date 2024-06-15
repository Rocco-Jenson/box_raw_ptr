/* Wrapper around malloc() and free() to remove libc dependency */
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

/* Check if architecture is x64 or ARM
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

volatile static uint8_t HEAP_COUNT = 0;

void* c_global_allocator(arch_type bytes) {   
    void* ptr = malloc(bytes);
    if (ptr != NULL) {
        HEAP_COUNT++;
    }
    /* return no matter if null required 
    for handle_alloc_error in rust */
    return ptr;
}

void c_global_deallocator(void* ptr) {
    if (ptr != NULL) {
        HEAP_COUNT--;
        free(ptr);
    }
}

// HEAP_COUNT = 0, arch_type = 1
const arch_type* global_allocation_info() {
    static arch_type arr[2];
    arr[0] = HEAP_COUNT;
    if (sizeof(arch_type) == sizeof(uint64_t) {
        arr[1] = 64;
    } else if (sizeof(arch_type) == sizeof(uint32_t) {
        arr[1] = 32;
    }
    return arr;
}
