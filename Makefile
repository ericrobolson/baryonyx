test: FORCE
	cargo test

test-watch: FORCE
	cargo watch -x test

installs: FORCE
	cargo binstall cargo-watch

docs: FORCE
	cargo doc --no-deps --open


example-hello-world: FORCE
	cargo run --example hello_world

FORCE:

