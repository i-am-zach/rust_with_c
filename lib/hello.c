#include <stdio.h>

int hello() {
    return 42;
}

int sum(int a, int b) {
    return a + b;
}

int mult(int a, int b) {
    return a * b;
}

int seg_fault() {
    int *p = 0;
    printf("%d", *p);
    return 42;
}