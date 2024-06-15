#include <stdlib.h>

int* get_c_ptr() {
    // Make memory allocation of 1 int
    int* ptr = (int*)malloc(sizeof(int));
    // Check if memory allocation failed
    int* val = (ptr != NULL) ? ptr : NULL;
    // Assign value to int* ptr
    *val = 12;
    // Return ptr with assigned value
    return val;
}
