# WASM target
rustup target add wasm32-unknown-unknown
# WASM Bindgen CLI
cargo install wasm-bindgen-cli
# Build the project
cargo build --release --target wasm32-unknown-unknown
# Setup target directory
mkdir public
# Move the index file
cp -r index.html public
# Move the assets
cp -r assets public
# Bind the wasm build
wasm-bindgen --out-dir public --target web target/wasm32-unknown-unknown/release/murg.wasm
