#!/bin/bash
# A handy wrapper for testing things.

rm -f output.yaml
cargo run -- -f test.log -m overwrite $@
