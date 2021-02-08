#!/bin/sh

# Builds the NPM package
wasm-pack build --release

# Run wasm-opt on binary for platforms where wasm-pack didn't do it for us automatically
# (currently M1 macs).
# We can safely run it again on a .wasm file that's already been optimized though.
which wasm-opt > /dev/null
if [ $? -eq 0 ]; then
    wasm-opt -Os -o pkg/jackvm_player_bg.new.wasm pkg/jackvm_player_bg.wasm
    mv pkg/jackvm_player_bg.new.wasm pkg/jackvm_player_bg.wasm
fi

# This is a hack. wasm-pack hard-codes `sideEffects` false into its generated
# package.json. Webpack 4+'s treeshaking breaks though, because it erronously
# treeshakes out files we need in this module.
cat ./pkg/package.json | jq '. + {"sideEffects": true}' > ./pkg/package.new.json
mv ./pkg/package.new.json ./pkg/package.json