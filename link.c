// linked to tmp.s
#include <stdio.h>
#include <stdint.h>

int foo() {
    printf("OK\n");
    return 0;
}

int dump1(uint64_t a) {
    printf("%d\n", a);
    return 0;
}

int dump2(uint64_t a, uint64_t b) {
    printf("%d, %d\n", a, b);
    return 0;
}

int dump3(uint64_t a, uint64_t b, uint64_t c) {
    printf("%d, %d, %d\n", a, b, c);
    return 0;
}

int dump4(uint64_t a, uint64_t b, uint64_t c, uint64_t d) {
    printf("%d, %d, %d, %d\n", a, b, c, d);
    return 0;
}

int hoge() {
    dump4(1,2,3, 4);
    return 0;
}