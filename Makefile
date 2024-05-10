run:
	cargo build & cargo run

build-wasm:
	cargo build -r --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/anniversary-game.wasm anniversary-game.wasm
