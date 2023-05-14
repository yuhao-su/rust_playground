#include<stdlib.h>

int test_alloc() {
    malloc(8 * (1<<20));
    return 0;
}