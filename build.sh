#!/usr/bin/env bash
docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
