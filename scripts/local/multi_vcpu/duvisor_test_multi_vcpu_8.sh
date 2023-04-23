#!/bin/bash

# trap ctrl-c and call ctrl_c()
trap ctrl_c INT

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
./scripts/expect_wrapper.sh ./scripts/local/multi_vcpu/duvisor_test_multi_vcpu_8.exp
ret=$?
pkill screen
exit $ret
