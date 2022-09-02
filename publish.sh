echo "Publishing to crates.io..."
cargo publish
echo "Publishing to npm..."
wasm-pack build --target web
node patch_wasm.js
wasm-pack pack
wasm-pack publish