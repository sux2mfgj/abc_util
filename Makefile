SRC := ./src/main.rs
ABC_NUM := 088

build: $(SRC)
	cargo build

sample:
	wget https://atcoder.jp/contests/abc$(ABC_NUM)/tasks/abc$(ABC_NUM)_a -O sample.html
