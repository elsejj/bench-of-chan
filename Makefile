

all: go rs

go: boc-go/main.go
	cd boc-go && go build -o target/boc-go

rs: boc-rs/src/main.rs
	cd boc-rs && cargo build --release