#include <stdlib.h>

int* get_c_ptr() {
    int* ptr = (int*)malloc(sizeof(int));

    if (ptr == NULL) {
        return NULL; // Memory Allocation Failed
    }

    return ptr;
}