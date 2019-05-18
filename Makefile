SRC := ./src/main.rs
ABC_NUM := 088

build: $(SRC)
	cargo build

release: $(SRC)
	cargo build --release --target=x86_64-unknown-linux-musl

test:
	cargo test

fmt:
	cargo fmt

clean:
	cargo clean
