default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32-unknown-unknown --release
	
optimize:
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/vitacare_institution_registry.wasm

clean:
	cargo clean 