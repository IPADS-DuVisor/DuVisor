#!/bin/bash

./qemu-duvisor/build/riscv64-softmmu/qemu-system-riscv64 \
    -snapshot \
    -nographic \
    -cpu rv64,x-h=true,x-z=true \
    -smp 4 \
    -m 8G \
    -machine virt \
    -bios ./opensbi-duvisor/build/platform/generic/firmware/fw_jump.elf \
    -kernel ./linux-duvisor/arch/riscv/boot/Image \
    -initrd ./prepare/rootfs-host.img \
    -append "root=/dev/ram0 rw console=ttyS0 earlycon=sbi"
