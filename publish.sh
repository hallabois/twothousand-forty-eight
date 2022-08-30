wasm-pack build --target web
node patch_wasm.js
wasm-pack pack
wasm-pack publish