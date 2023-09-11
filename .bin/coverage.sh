#!/bin/bash

RUSTFLAGS="-C instrument-coverage" cargo test formatjson5

rust-profdata merge -sparse default_*.profraw -o json5format.profdata 

rust-cov report \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=json5format.profdata \
    --object target/debug/deps/web-fdd1bc4cc2a7e4ce

rust-cov show \
    --use-color --ignore-filename-regex='/.cargo/registry' \
    --instr-profile=json5format.profdata \
    --object  target/debug/deps/web-fdd1bc4cc2a7e4ce \
    --show-instantiations --show-line-counts-or-regions \
    --Xdemangler=rustfilt | less -R