cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/roguelikefontviewer.wasm --out-dir build/web/ --no-modules --no-typescript
