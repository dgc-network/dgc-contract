# Copyright (c) The dgc.network
# SPDX-License-Identifier: Apache-2.0

FROM ubuntu:bionic

RUN apt-get update \
  && apt-get install gnupg -y

# Need sawtooth-sdk to create keys
RUN echo "deb http://repo.sawtooth.me/ubuntu/nightly bionic universe" >> /etc/apt/sources.list \
 && apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 44FC67F19B2466EA \
 && apt-get update \
 && apt-get install -y -q \
    curl \
    gcc \
    libpq-dev \
    libssl-dev \
    libzmq3-dev \
    pkg-config \
    python3-sawtooth-cli \
    python3-sawtooth-sdk \
    unzip \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

# For Building Protobufs
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
 && curl -OLsS https://github.com/google/protobuf/releases/download/v3.5.1/protoc-3.5.1-linux-x86_64.zip \
 && unzip protoc-3.5.1-linux-x86_64.zip -d protoc3 \
 && rm protoc-3.5.1-linux-x86_64.zip


WORKDIR /project/cli/

ENV PATH=$PATH:/protoc3/bin:/root/.cargo/bin:/project/cli/target/debug/
