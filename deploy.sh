#!/usr/bin/env bash

# WASM_PATH="$(find ./target/wasm32-unknown-unknown/release/ -maxdepth 1 -name "*.wasm")"

# near deploy \
#   --wasmFile $WASM_PATH \
#   --accountId "kenobi.tesnet" \
#   --initFunction new \
#   --initArgs "$(node ./init-args.js)"


WASM_PATH="./res/pet_shop_contract.wasm"
PET_CONTRACT="petshop.kenobi.testnet"
near create-account $PET_CONTRACT --masterAccount kenobi.testnet 
near deploy --accountId $PET_CONTRACT --wasmFile $WASM_PATH --initFunction "new" --initArgs '{}'

