#!/bin/sh

gdb -ex 'target remote :1234' ./target/x86_64-rust_os/debug/rust_os
