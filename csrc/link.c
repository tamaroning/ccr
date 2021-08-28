// linked to tmp.s
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

int dump(uint32_t a) {
    printf("%d\n", a);
    return 0;
}

int dump2(uint32_t a, uint32_t b) {
    printf("%d, %d\n", a, b);
    return 0;
}

int square(uint32_t a) {
    return a*a;
}

int mysleep(unsigned int t) {
    usleep(t);
    return 0;
}

int do_nothing(void){
    return 0;
}
