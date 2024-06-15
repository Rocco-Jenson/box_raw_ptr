#include <stdlib.h>

int* get_c_ptr() {
    int* ptr = (int*)malloc(sizeof(int));
    // Check if memory allocation Failed
    int* val = (ptr != NULL) ? ptr : NULL;
    // Assign value to int* ptr
    *val = 12;
    return val;
}
