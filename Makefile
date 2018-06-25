build:
	cargo +nightly build --features clippy

publish:
	cargo package
	cargo publish