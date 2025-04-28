default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release

clean:
	cargo clean
	rm -f target/wasm32-unknown-unknown/release/*.wasm

.PHONY: test build clean 