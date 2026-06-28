FILE ?= Code.MC

run:
	clear
	cargo run $(FILE)
build:
	clear
	cargo build
check:
	clear
	cargo check
debug:
	cargo build