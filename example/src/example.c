#include <stdlib.h>

int* get_c_ptr() {
    /* Make memory allocation of 1 int */
    int* ptr = (int*)malloc(sizeof(int));
    /* Check if memory allocation failed */
    if (ptr == NULL) { exit(EXIT_FAILURE); }
    /* Assign value to int* ptr */
    *ptr = 12;
    /* Return ptr with assigned value */
    return ptr;
}