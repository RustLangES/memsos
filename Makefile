build:
	cargo build

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt

test:
	cargo test