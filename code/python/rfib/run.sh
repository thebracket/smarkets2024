#!/bin/bash
cargo build --release
cp ../../target/release/librfib.so .
python3 fib.py
