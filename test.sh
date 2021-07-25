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

assert 6 'a = 1;
b = 9/3-1;
c = -1 * (4-7);
d = a + b + c;'

echo OK
