#!/bin/bash

# trap ctrl-c and call ctrl_c()
trap ctrl_c INT

export PREPARE=

function ctrl_c() {
	pkill screen
}


screen -S virt -d -m &
VIRT_PID=$!
echo "virt screen session pid is ${VIRT_PID}"
screen -S host -d -m &
HOST_PID=$!
echo "host screen session pid is ${HOST_PID}"
sleep 1
./scripts/expect_wrapper.sh ./scripts/local/duvisor_test.exp
ret=$?
pkill screen
exit $ret
