#!/bin/bash

CI_HOSTNAME=ip-172-31-63-27

if [ $(hostname)1 == ${CI_HOSTNAME}1 ]; then
    # for CI environment
    PREPARE="/home/ubuntu/prepare"
else
    PREPARE="./prepare"
fi

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
mkdir -p mnt
sudo mount $PREPARE/ubuntu-vdisk.img ./mnt
sudo rm -r ./mnt/duvisor
sudo mkdir -p ./mnt/duvisor/tests_bin
sudo cp scripts/local/run_tests.sh $duvisor_name ./mnt/duvisor
sudo cp $duvisor_test_names ./mnt/duvisor/tests_bin/
sudo cp -r src ./mnt/duvisor/
sudo cp -r tests ./mnt/duvisor/
sudo cp -r test-files-duvisor ./mnt/duvisor/test-files-duvisor

sudo umount ./mnt
