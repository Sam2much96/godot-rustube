#!/usr/bin/env bash

export C_INCLUDE_PATH="/home/samuel/glibc-2.31"

#export OPENSSL="build_openssl_android_clang/openssl-1.1.1l"

#export OPENSSL_DIR='/home/samuel/'$OPENSSL

#export OPENSSL_LIB_DIR="/home/samuel/"$OPENSSL
#export PKG_CONFIG_PATH="/home/samuel/"$OPENSSL

RUST_BACKTRSCE=full cargo build  --target x86_64-unknown-linux-gnu  --release 