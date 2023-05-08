#!/bin/bash

# Reference: https://github.com/IPADS-DuVisor/kvm-tutorial.

export ARCH=riscv
export CROSS_COMPILE=riscv64-linux-gnu-

CI_HOSTNAME=

if [ $(hostname)1 == ${CI_HOSTNAME}1 ]; then
    # for CI environment
    PREPARE="/home/ubuntu/prepare"
else
    PREPARE="./prepare"
fi

echo prepare dirctory is ${PREPARE}
# We would user this to build rootfs of both guest and host.
ROOTFS_DIR=${PREPARE}/rootfs
if [ ! -e $ROOTFS_DIR ]; then
	echo "Root fs not found for $ROOTFS_DIR !"
	exit 0
fi
pushd $ROOTFS_DIR
cp -f ../howto/configs/busybox-1.33.1_defconfig .config
make oldconfig
make install -j$(nproc)
chmod 777 _install
mkdir -p _install/etc/init.d
mkdir -p _install/dev
mkdir -p _install/proc
mkdir -p _install/sys
mkdir -p _install/apps
ln -sf /sbin/init _install/init
cp -f ../howto/configs/busybox/fstab _install/etc/fstab
cp -f ../howto/configs/busybox/rcS _install/etc/init.d/rcS
cp -f ../howto/configs/busybox/motd _install/etc/motd
pushd _install
find ./ | cpio -o -H newc > ../../rootfs-guest.img
popd
popd
