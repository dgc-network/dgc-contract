# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM ubuntu:bionic

RUN apt-get update \
 && apt-get install -y -q \
 curl \
 gcc \
 libssl-dev \
 libzmq3-dev \
 pkg-config \
 unzip \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# For Building Protobufs
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
 && /root/.cargo/bin/rustup default nightly \
 && curl -OLsS https://github.com/google/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip \
 && unzip protoc-3.5.1-linux-x86_64.zip -d protoc3 \
 && rm protoc-3.5.1-linux-x86_64.zip

RUN /root/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain nightly

WORKDIR /project/examples
