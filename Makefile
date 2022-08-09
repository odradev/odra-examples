prepare:
	sudo apt install wabt
	rustup target add wasm32-unknown-unknown
	cargo install cargo-odra

test:
	cargo odra test
	cargo odra test -b casper

clippy:
	cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

clean:
	cargo clean
