#!/bin/bash
pushd opensbi-duvisor

export CROSS_COMPILE=riscv64-linux-gnu-
make PLATFORM=generic
if [ $? -ne 0 ]; then
    exit -1
fi

popd
