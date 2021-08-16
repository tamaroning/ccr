// linked to tmp.s
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

int dump(uint64_t a) {
    printf("%d\n", a);
    return 0;
}

int dump2(uint64_t a, uint64_t b) {
    printf("%d, %d\n", a, b);
    return 0;
}

int square(uint64_t a) {
    return a*a;
}

int mysleep(unsigned int t) {
    usleep(t);
    return 0;
}

int do_nothing(void){
    return 0;
}
