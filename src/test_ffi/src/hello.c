#include<stdlib.h>

int test_alloc() {
    malloc(2 * 1<<30);
    return 0;
}