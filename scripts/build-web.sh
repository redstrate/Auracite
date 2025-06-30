#!/bin/sh

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web --release --no-pack --no-typescript
