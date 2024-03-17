#!/bin/bash
cargo build --release
cp target/release/librfib.dylib ./librfib.so
python3 fib.py
