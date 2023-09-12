#!/bin/bash

gzip -9 < ./pkg/stringify_bg.wasm | cat >> ./pkg/stringify_bg.wasm.gzip