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
    
    echo "Run ./tmp"
    echo "------- Result -------"
    if [ "$actual" = "$expected" ]; then
        echo "Got $actual as expected"
    else
        echo "$expected is expected, but got $actual"
        exit 1
    fi
}

assert 9 '
sum = 0;
width = 3;
for ( i = 1; i <= width; i = i + 1) {
    for (j = 1; j <= width; j = j + 1) {
        sum = sum + 1;
    }
}
return sum;
'

echo OK
