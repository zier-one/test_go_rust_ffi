package main


//#cgo LDFLAGS: -L${SRCDIR}/target/debug -lrusttest
/*
extern void rust_simple_fn();
extern void rust_run_callback(void (*func)(int));
extern void rust_notify();
*/
import "C"
import "fmt"
import "golang.org/x/sys/unix"



//export GoSimpleFn
func GoSimpleFn() {
    fmt.Println("Go函数被Rust调用")
}

//export Callback
func Callback(a int) {
	fmt.Println("Callback",a)
}

var ch = make(chan struct{})

func runWaitGoroutine() {
	fmt.Println("runWaitGoroutine wait")
	<-ch
	fmt.Println("runWaitGoroutine done")
}


//export GoWakeUpGoroutine
func GoWakeUpGoroutine() {
    ch<-struct{}{}
}



func main() {

    fmt.Printf("POSIX Thread ID: %v\n", unix.Gettid())
    C.rust_simple_fn()

	// 函数指针方式调用不成功
	// p:=func(a int){
	// 	Callback(a)
	// }
	// C.rust_run_callback((*[0]byte)(unsafe.Pointer(&p)))
	C.rust_notify()
	runWaitGoroutine()
}