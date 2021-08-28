#!/bin/bash
TEST_DIR="$(cd $(dirname $0); pwd)/"
CCR_DIR="${TEST_DIR}../"
DEBUG="${CCR_DIR}target/debug/"

assert() {
    expected="$1"
    input="$2"

    echo "$input" > "${DEBUG}"tmp.src
    "${DEBUG}"ccr "${DEBUG}tmp.src"
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

assert 10 '{int a = 10;return *(&a);}'
assert 2 '{int a = 1;int b = 2;return *(&a-4);}'
assert 6 '{int a = 5;int b;*(&a-4)=6;return b;}'

echo OK
