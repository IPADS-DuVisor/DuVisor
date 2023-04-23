#!/bin/bash

./scripts/build/build_qemu.sh
./scripts/build/build_host_linux.sh
./scripts/build/build_guest_linux.sh
./scripts/build/build_opensbi.sh
./scripts/build/build_rootfs.sh
./scripts/build/build_duvisor.sh
./scripts/build/example_copy.sh
