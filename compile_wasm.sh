#!/bin/sh

set -ex

# Compile our wasm module and run `wasm-bindgen`
wasm-pack build --release --features wasm

# Run the `wasm2js` tool from `binaryen`
#wasm2js pkg/twothousand_forty_eight_bg.wasm -o pkg/twothousand_forty_eight_bg.wasm.js

# Update our JS shim to require the JS file instead
#sed -i 's/twothousand_forty_eight_bg.wasm/twothousand_forty_eight_bg.wasm.js/' pkg/twothousand_forty_eight.js

# Run patch_wasm.js to patch the wasm file
node patch_wasm.js