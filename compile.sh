#!/bin/bash
DEBUG="target/debug/"

./"${DEBUG}"ccr "$1"
echo "tmp.s is created at $DEBUG"
