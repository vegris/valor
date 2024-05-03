.PHONY: check

# Use nightly formatter because 'imports_granularity' and 'group_imports' settings are unstable

fmt:
	cargo +nightly fmt

check:
	cargo +nightly fmt --check
	cargo check
	cargo clippy

run:
	cargo run

