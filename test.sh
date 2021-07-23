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

assert 0 0
assert 42 42
assert 6 "1+(2+3)"
assert 37 "1+2*(3-4)*5+6*7+8/2"
assert 20 "-2*3+(+5*2)*2-3*-2"

echo OK
