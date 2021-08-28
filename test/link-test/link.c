// linked to tmp.s
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

int dump_uint32(uint32_t a) {
    printf("%u\n", a);
    return 0;
}

int dump_int32(int32_t a) {
    printf("%d\n", a);
    return 0;
}

int dump_uint64(uint64_t a) {
    printf("%u\n", a);
    return 0;
}

int dump_int64(int64_t a) {
    printf("%d\n", a);
    return 0;
}

int dump_addr(uint64_t a) {
    printf("%016x\n", a);
}

int square(uint32_t a) {
    return a*a;
}

int mysleep(uint32_t t) {
    usleep(t);
    return 0;
}

int do_nothing(void){
    return 0;
}
