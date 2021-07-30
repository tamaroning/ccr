// linked to tmp.s
#include <stdio.h>
#include <stdint.h>

int dump(uint64_t a) {
    printf("%d\n", a);
    return 0;
}

int square(uint64_t a) {
    return a*a;
}