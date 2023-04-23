#!/bin/bash

./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_qemu.sh
./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_host_linux.sh
./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_guest_linux.sh
./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_opensbi.sh
./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_rootfs.sh
./scripts/build/docker_exec_wrapper.sh ./scripts/build/build_duvisor.sh
./scripts/build/example_copy.sh

