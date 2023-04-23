#!/bin/bash

# Execute expect
echo $1
$1 | tr "\r" "\n"

# Check Return value
if [ ${PIPESTATUS[0]} -eq 0 ]; then
        exit 0
else
        exit -1
fi
