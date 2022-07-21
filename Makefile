prepare:
	sudo apt install wabt
	rustup target add wasm32-unknown-unknown

test:
	cargo test

clippy:
	cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
