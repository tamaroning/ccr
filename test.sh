#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    ./ccr "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

assert 37 "1 +2*(3-4)*5+6*7+8/2"
assert 17 " -2 * 3+(+5 *2 ) *2-3 *-2 +9 /- 3"
assert 1 "1*(-3-8)/4+1 < 4*(5+2)"
assert 0 "1+1== 3>4"

echo OK
