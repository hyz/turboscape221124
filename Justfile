####!/usr/bin/env just --working-directory . --justfile
# vim: set ft=make :

wasm-pack-greet:
	cd apps/greet
	wasm-pack build . --target web --out-name index --out-dir ../../dist/greet

initial-turbo:
	yarn
	yarn build

initial-tauri:
	cargo check
	cargo run

