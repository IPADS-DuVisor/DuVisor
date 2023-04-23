#!/bin/bash

if [ ! -e "$HOME/.cargo/config" ]; then
mkdir -p ~/.cargo/
echo '
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

[net]
git-fetch-with-cli = true

[target.riscv64gc-unknown-linux-gnu]
linker = "riscv64-linux-gnu-gcc"
' >> ~/.cargo/config

# If `~/.cargo/config` does not exist, it is likely to be in docker env.
# In docker env, `rustup update` would encounter error.
# According to https://github.com/rust-lang/rustup/issues/2729#issuecomment-1516103534,
# we reinstall the rustup stable toolchain to keep it up-to-date.
rustup toolchain uninstall stable && rustup toolchain install stable
fi

export PATH=/root/.cargo/bin:$PATH

first_arg=${1:-release}

if test ${first_arg} = release; then
    build_level="--release"
    build_path=release
elif test ${first_arg} = debug; then
    build_level=""
    build_path=debug
else
    echo "Wrong arg."
    exit
fi

#export CARGO_NET_OFFLINE=true

echo `hostname`

echo $build_level

cargo clean
cargo update
rustup default stable
rustup update
rustup target add riscv64gc-unknown-linux-gnu
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target=riscv64gc-unknown-linux-gnu $build_level --features "xilinx"


# get duvisor all the binary names
RUSTFLAGS='-C target-feature=+crt-static' cargo test --no-run --target=riscv64gc-unknown-linux-gnu $build_level --features "xilinx"

## Build test images
rm -r ./tests/integration/test_images/build
./tests/integration/test_images/build.sh ./tests/integration/test_images/build ./tests/integration/
