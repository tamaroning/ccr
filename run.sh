#!/bin/bash

DEBUG="target/debug"

./"${DEBUG}"/ccr "$1"
cc test/link.c "${DEBUG}"/tmp.s -o "${DEBUG}"/a.out
./"${DEBUG}"/a.out
