check-clippy: 
	cargo clippy -- -D warnings
	cargo clippy --all-features --tests -- -D warnings

check-fmt:
	cargo fmt -- --check


build:
	cargo build --target wasm32-wasi