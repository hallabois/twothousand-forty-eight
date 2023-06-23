# twothousand-forty-eight

a basic engine and move validator for the game 2048

This is a library written in rust, the documentation can be found at [docs.rs](https://docs.rs/twothousand-forty-eight/).

The most up-to-date version can be downloaded from [crates.io](https://crates.io/crates/twothousand-forty-eight) or the wasm version from [npm](https://www.npmjs.com/package/twothousand-forty-eight).

See [LICENSE](LICENSE) for information on the used license (MIT).

## How-tos

### Testing

Run `cargo test` to run the default tests.

Run `cargo test -- --ignored` instead to run expensive tests (such as validating 10 000 real games)

### Building

Run `cargo build --release` to build

### Publishing

Run the provided publish.sh to publish the package to crates.io and the wasm bindings to npm. You need to have wasm-pack installed in order to build the npm package.
