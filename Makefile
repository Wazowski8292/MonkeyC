FILE ?= Code.MC

run:
	clear
	cargo run $(FILE)
	./Code
build:
	clear
	cargo build
check:
	clear
	cargo check
debug:
	cargo build

clean: 
	cargo clean