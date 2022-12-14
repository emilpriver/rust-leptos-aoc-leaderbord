run:
	cargo +nightly run --target wasm32-unknown-unknown

run-server:
	wasm-pack build --target=web --no-default-features --features=hydrate && cargo +nightly run --no-default-features --features=ssr

dev:
	trunk serve --open
