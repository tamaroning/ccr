int main(void) {
    int a;
    a = 100;
    dump_int64(a);
    int i = 0;
    for (i = 0; i <= 10; i = i+1) {
        a = a+ i;
    }
    dump_int64(a);
    return 0;
}

int func(void) {
    int a,b,c,d;
    return 0;
}