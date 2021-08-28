#!/bin/bash
CCR_DIR="$(cd $(dirname $0); pwd)/"
DEBUG="${CCR_DIR}target/debug/"

"${DEBUG}"ccr "$1"
cc "${CCR_DIR}"csrc/link.c "${DEBUG}"tmp.s -o "${DEBUG}"tmp
"${DEBUG}"tmp
