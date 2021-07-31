#!/bin/bash

./ccr "$1"
cc link.c tmp.s
./a.out