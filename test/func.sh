#!/bin/bash
TEST_DIR="$(cd $(dirname $0); pwd)/"
CCR_DIR="${TEST_DIR}../"
DEBUG="${CCR_DIR}target/debug/"

assert() {
    expected="$1"
    input="$2"

    echo "$input" > "${DEBUG}"tmp.src
    "${DEBUG}"ccr -q "${DEBUG}tmp.src"
    cc -o ${DEBUG}tmp "${DEBUG}"tmp.s
    ${DEBUG}tmp
    actual="$?"
    
    if [ "$actual" = "$expected" ]; then
        echo -n "."
        #echo "Got $actual as expected"
    else
        echo -e "\n$expected is expected, but got $actual"
        echo -e "Input:\n$input"
        exit 1
    fi
}

assert 30 'int main(void) { return 30; }'
assert 3 'int ret1(void) { return 1; } int ret2(void) { return 2; } int main(void) { return ret1() + ret2(); } '
assert 0 'void func(void) { return; } int main(void) { func(); return 0; }'
assert 13 'int fib(int n) {
    if (n == 1) return 1;
    if (n == 2) return 1;
    return fib(n-1) + fib(n-2);
}
int main(void) {
    return fib(7);
}
'
echo OK
