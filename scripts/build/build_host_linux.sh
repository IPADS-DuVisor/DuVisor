#!/bin/bash
pushd linux-duvisor

cp .config-qemu .config
export ARCH=riscv
export CROSS_COMPILE=riscv64-linux-gnu-

make -j$(nproc)

if [ $? -ne 0 ]; then
    exit -1
fi

popd
