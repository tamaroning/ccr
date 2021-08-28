#!/bin/bash
TEST_DIR="$(cd $(dirname $0); pwd)/"

${TEST_DIR}culc.sh
${TEST_DIR}pointer.sh

echo OK