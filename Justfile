build-wasm:
    rustup target add wasm32-unknown-unknown && \
    cargo build --release --target wasm32-unknown-unknown -p wasm && \
    mkdir -p game-of-life-web/wasm && \
    wasm-bindgen --target web --out-dir game-of-life-web/wasm target/wasm32-unknown-unknown/release/wasm.wasm