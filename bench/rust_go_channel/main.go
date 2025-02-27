package main

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lgoodboy
/*
extern void rust_add_async(int a, int b, uintptr_t ch);

*/
import "C"
import "unsafe"

func main() {
	callRustAsync(1, 2)

}

func callRustAsync(a, b int) int {
	doneCh := make(chan int)
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
