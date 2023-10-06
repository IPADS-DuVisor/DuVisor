FROM ubuntu:22.04
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=Asia/Shanghai

# Build essentials
RUN apt-get update && apt-get install -y \
            ca-certificates build-essential \
            curl git libssl-dev \
            pkg-config python3 wget \
            qemu-system-misc opensbi u-boot-qemu qemu-utils \
            gcc-riscv64-linux-gnu

# Install Rust
RUN mkdir -p /tmp
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rust_installer.sh
RUN chmod +x /tmp/rust_installer.sh
RUN /tmp/rust_installer.sh -y

# Install packages for compiling qemu and linux
RUN apt-get update && apt-get install -y ninja-build libglib2.0-dev libpixman-1-dev flex bison bc

# Install cmake for cargo build
RUN apt-get update && apt-get install -y cmake

# Install cpio for rootfs build
RUN apt-get update && apt-get install -y cpio
