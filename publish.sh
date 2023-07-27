echo "Publishing to crates.io..."
cargo publish
echo "Publishing to npm..."
./compile_wasm.sh
wasm-pack pack
wasm-pack publish