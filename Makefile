check-clippy: 
	cargo clippy -- -D warnings
	cargo clippy --all-features --tests -- -D warnings

build:
	cargo build --target wasm32-wasi