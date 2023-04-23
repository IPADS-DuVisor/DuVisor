#!/bin/bash
pushd ./linux-guest

export ARCH=riscv
export CROSS_COMPILE=riscv64-linux-gnu-

make defconfig
make -j$(nproc)

popd
