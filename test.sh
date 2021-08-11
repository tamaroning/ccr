#!/bin/bash

DEBUG="target/debug"

assert() {
    expected="$1"
    input="$2"

    echo "------- Input -------"
    # echo "$input"
    #echo "EOF"

    echo "$input" > tmp.src
    ./"${DEBUG}"/ccr "${DBEUG}"/tmp.src
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

assert 11 '
int sum = 0;
int a = sum +11, b, c;
return a;
'

echo OK
