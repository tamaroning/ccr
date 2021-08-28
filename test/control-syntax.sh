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

assert 55 '{ int i; int sum = 0; for(i = 0; i <= 10; i = i + 1) { sum = sum + i; } return sum; }'
assert 200 '{ int flag = 1; if (flag == 1) { return 200; } else return 100; }'

echo OK