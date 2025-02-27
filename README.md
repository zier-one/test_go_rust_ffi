```
cargo build --release

# ffi
GO111MODULE=on CGO_ENABLED=1 go run ./bench/ffi_base/main.go

# rust go channel
GO111MODULE=on CGO_ENABLED=1 go run ./bench/rust_go_channel/main.go
```