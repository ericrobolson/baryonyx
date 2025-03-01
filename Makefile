test: FORCE
	cargo test

test-watch: FORCE
	cargo watch -x test

installs: FORCE
	cargo binstall cargo-watch

docs: FORCE
	cargo doc --no-deps --open

FORCE:

