#!/bin/bash
pushd ./qemu-duvisor

if [ ! -e "./build/Makefile" ]; then
    ./configure --target-list=riscv64-softmmu
fi

make -j $(nproc)

popd
