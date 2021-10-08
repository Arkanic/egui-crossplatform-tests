set -eu

rustup target install wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
cargo update -p wasm-bindgen

cargo install basic-http-server