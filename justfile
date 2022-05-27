default:
  just --list

build:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --target web --no-typescript target/wasm32-unknown-unknown/release/rustpython_bug.wasm --out-dir www

serve:
  python3 -m http.server --directory www --bind 0.0.0.0
