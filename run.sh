#!/bin/bash
CCR_DIR="$(cd $(dirname $0); pwd)/"
DEBUG="${CCR_DIR}target/debug/"

"${DEBUG}"ccr -q ${CCR_DIR}test/link-test/test.c #"$1"
cc "${CCR_DIR}"test/link-test/link.c "${DEBUG}"tmp.s -o "${DEBUG}"tmp
"${DEBUG}"tmp
