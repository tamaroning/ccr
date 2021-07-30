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
    
    #echo "Run ./tmp"
    echo "------- Result -------"
    if [ "$actual" = "$expected" ]; then
        echo "Got $actual as expected"
    else
        echo "$expected is expected, but got $actual"
        exit 1
    fi
}

assert 3 '{ x=3; return *&x; }'
assert 3 '{ x=3; y=&x; z=&y; return **z; }'
assert 5 '{ x=3; y=&x; *y=5; return x; }'

assert 7 '{ x=3; y=5; *(&x-8)=7; return y; }'
assert 7 '{ x=3; y=5; *(&y+8)=7; return x; }'

assert 5 '{ x=3; y=5; return *(&x-8); }'
assert 3 '{ x=3; y=5; return *(&y+8); }'

echo OK
