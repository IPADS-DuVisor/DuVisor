#!/bin/bash

PREPARE="./prepare"

echo prepare dirctory is ${PREPARE}

if [ ! -e $PREPARE ]; then
    echo "Prepare dirctory not exist!"
    exit -1
fi

first_arg=${1:-release}

if test ${first_arg} = release; then
    build_path=release
elif test ${first_arg} = debug; then
    build_path=debug
else
    echo "Wrong arg."
    exit
fi

duvisor_name=./target/riscv64gc-unknown-linux-gnu/${build_path}/duvisor

deps=`ls ./target/riscv64gc-unknown-linux-gnu/${build_path}/deps/*`

# Delete duvisor main binary name, so that we get duvisor tests binary names
for i in $deps; do
    [[ ! `diff $i $duvisor_name` ]] && sudo rm $i
done

duvisor_names=`find ./target/riscv64gc-unknown-linux-gnu/${build_path}/deps/ -type f ! -name '*.*' `

duvisor_test_names=${duvisor_names/$duvisor_name}
ROOTFS_DIR=$PREPARE/rootfs
rm -rf $ROOTFS_DIR/_install/duvisor
mkdir -p $ROOTFS_DIR/_install/duvisor
cp $duvisor_name $ROOTFS_DIR/_install/duvisor/
cp $PREPARE/rootfs-guest.img $ROOTFS_DIR/_install/duvisor
cp ./linux-guest/arch/riscv/boot/Image $ROOTFS_DIR/_install/duvisor

echo '#!/bin/bash
./duvisor \
--smp 2 \
--initrd \
./rootfs-guest.img \
--kernel ./Image \
--memory 512 \
--machine duvisor_virt \
--append "root=/dev/ram console=ttyS0 earlycon=sbi"
' > $ROOTFS_DIR/_install/duvisor/boot.sh

chmod +x $ROOTFS_DIR/_install/duvisor/boot.sh

pushd $ROOTFS_DIR/_install
find ./ | cpio -o -H newc > ../../rootfs-host.img
popd
