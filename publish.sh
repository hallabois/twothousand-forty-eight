echo "Publishing to crates.io..."
cargo publish
echo "Publishing to npm..."
wasm-pack build --release --features wasm
node patch_wasm.js
wasm-pack pack
wasm-pack publish