#!/bin/sh

# Builds the NPM package
wasm-pack build --release

# This is a hack. wasm-pack hard-codes `sideEffects` false into its generated
# package.json. Webpack 4+'s treeshaking breaks though, because it erronously
# treeshakes out files we need in this module.
cat ./pkg/package.json | jq '. + {"sideEffects": true}' > ./pkg/package.new.json
mv ./pkg/package.new.json ./pkg/package.json