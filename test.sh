#!/bin/bash
CCR_DIR="$(cd $(dirname $0); pwd)/"
DEBUG="${CCR_DIR}target/debug/"

assert() {
    expected="$1"
    input="$2"

    echo "$input" > "${DEBUG}"tmp.src
    "${DEBUG}"ccr -d "${DEBUG}tmp.src"
    cc -o ${DEBUG}tmp "${DEBUG}"tmp.s
    ${DEBUG}tmp
    actual="$?"
    
    if [ "$actual" = "$expected" ]; then
        echo "Got $actual as expected"
    else
        echo -e "\n$expected is expected, but got $actual"
        echo -e "Input:\n$input"
        exit 1
    fi
}

assert 10 '
return 1+5+4;
'

echo OK
