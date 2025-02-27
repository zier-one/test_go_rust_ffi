package main

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lgoodboy
/*
#include <stdint.h>
extern void rust_add_async(int a, int b, uintptr_t ch);
extern void init_tokio_runtime();

*/
import "C"
import (
	"fmt"
	"time"
	"unsafe"
)

func main() {
	C.init_tokio_runtime()

	const numCalls = 100_000
	start := time.Now()
	for i := 0; i < numCalls; i++ {
		_ = callRustAsync(i, 2)

	}
	elapsed := time.Since(start)
	fmt.Printf("异步调用: Go -> Rust 调用次数: %d, 平均耗时: %v\n", numCalls, elapsed/time.Duration(numCalls))
}

func callRustAsync(a, b int) int {
	doneCh := make(chan int, 1)
	doneChPtr := unsafe.Pointer(&doneCh)
	C.rust_add_async(C.int(a), C.int(b), C.uintptr_t(uintptr(doneChPtr)))
	resp := <-doneCh
	return resp
}

//export PushChannel
func PushChannel(resp C.int, doneChPtr C.uintptr_t) {
	doneCh := *(*chan int)(unsafe.Pointer(uintptr(doneChPtr)))
	doneCh <- int(resp)
}
