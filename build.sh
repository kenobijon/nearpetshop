#!/usr/bin/env bash

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/pet_shop_contract.wasm ./res/
