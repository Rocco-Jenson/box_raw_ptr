// Wrapper around malloc() and free() to remove libc dependency
#include <stdlib.h>
#include <stdint.h>

#define X86_X64_SIZE 0xFFFFFFFFFFFFFFFFu // Max 64 bit integer
#define X86_SIZE  0xFFFFFFFF // Max 32 bit integer

// Check if architecture is x86_x64
#if UINTPTR_MAX == X86_X64_SIZE
    typedef uint64_t arch_type;
// Check if architecture is x86
#elif UINTPTR_MAX == X86_SIZE
    typedef uint32_t arch_type;
#else
    #error "Unsupported architecture: Only 64 and 32 bit architectures supported"
#endif

void* c_global_allocator(arch_type bytes) {
    // Return the pointer even if it's NULL to let Rust handle allocation errors
    return malloc(bytes);
}

void c_global_deallocator(void* ptr) {
    // Rust allocator manages NULL check
    free(ptr);
}
