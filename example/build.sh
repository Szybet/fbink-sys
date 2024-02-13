#!/bin/bash

# https://users.rust-lang.org/t/how-to-install-armv7-unknown-linux-musleabihf/82395/12
# rustup target add armv7-unknown-linux-musleabihf
CC=clang cargo build -vv --release --target=armv7-unknown-linux-musleabihf

# It's in target/armv7-unknown-linux-musleabihf/release/fbink-sys-example, scp it and run

# scp target/armv7-unknown-linux-musleabihf/release/fbink-sys-example root@10.42.0.28:/