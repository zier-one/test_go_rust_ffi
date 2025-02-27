```
cargo build --release

# ffi
GO111MODULE=on CGO_ENABLED=1 go run ./bench/ffi_base/main.go

# tokio & go channel
```