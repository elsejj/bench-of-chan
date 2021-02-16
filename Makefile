

all: go rs

go: boc-go/main.go
	cd boc-go && go build -o target/boc-go

rs: boc-rs/src/main.rs
	cd boc-rs && cargo build --release


run: go rs
	boc-go/target/boc-go -w 2000 -e 1000
	boc-rs/target/release/boc-rs -w 2000 -e 1000