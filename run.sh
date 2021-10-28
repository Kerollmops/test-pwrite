#!/usr/bin/env sh

rm -f big-file
mkfile -n 16818044928 big-file
cargo run --release
rm -f big-file
