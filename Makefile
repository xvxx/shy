.PHONY: build
build: src/*.rs
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: release
release: src/*.rs
	cargo build --release
