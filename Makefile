.PHONY: debug
debug: src/*.rs
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: build
build: src/*.rs
	cargo build --release

.PHONY: clean
clean:
	@rm -rf target

# Build manual
.PHONY: manual
manual: doc/shy.1

doc/shy.1: doc/shy.1.md scdoc
	scdoc < doc/shy.1.md > doc/shy.1

# Must have scdoc installed to build manual.
scdoc:
	@which scdoc || (echo "scdoc(1) not found."; \
		echo "please install to build the manpage: https://repology.org/project/scdoc"; exit 1)
