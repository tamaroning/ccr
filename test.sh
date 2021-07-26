#!/bin/bash

assert() {
    expected="$1"
    input="$2"

    echo "------- Input -------"
    # echo "$input"
    #echo "EOF"

    echo "$input" > tmp.src
    ./ccr tmp.src
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

    echo "------- Result -------"
    if [ "$actual" = "$expected" ]; then
        echo "Got $actual as expected"
    else
        echo "$expected is expected, but got $actual"
        exit 1
    fi
}

assert 6 'a = 1;
b = 9/3-1;
c = -1 * (4-7);
d = a + b + c;'

echo OK
