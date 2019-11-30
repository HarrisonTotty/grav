#!/bin/bash
# A handy wrapper for testing things.

rm -f output.yaml
cargo run --release -- -f test.log -m overwrite $@
