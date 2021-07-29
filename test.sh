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

assert 100 '
sum = 0;
for ( i = 0; i <= 10; i = i + 1)
    sum = sum + i;

if (sum > 50) return 100;
else return 200;
'

assert 100 '
i = 0;
while (i != 100)
    i = i + 1;
return i;
'

echo OK
