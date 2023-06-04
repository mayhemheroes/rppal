FROM rustlang/rust:nightly as builder

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y build-essential clang-11 llvm-11-tools cmake curl binutils-dev libbfd-dev libunwind8-dev libblocksruntime-dev pkg-config libudev-dev libssl-dev && \
    ln -s /usr/bin/llvm-config-11 /usr/bin/llvm-config

RUN rustup component add rust-src
RUN cargo install -f honggfuzz

ADD . /rppal
WORKDIR /rppal

#RUN mv inject/mod.rs src/pwm/mod.rs
RUN cd ./src && sed -i 's/mod sysfs/pub mod sysfs/g' pwm.rs

RUN cd ./fuzz && cargo +nightly hfuzz build

# Move Fuzzers to a clean image
FROM ubuntu:20.04

COPY --from=builder rppal/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/user_to_uid /
COPY --from=builder rppal/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/group_to_gid /