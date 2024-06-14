/* Wrapper around malloc() and free() to remove libc dependency */

#include <stdlib.h>

void* c_global_allocator(size_t bytes) {
    return malloc(bytes);
}

void c_global_deallocator(void* ptr) {
    if (ptr != NULL) {
        free(ptr);
    }
}