package main

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lgoodboy
/*
extern int rust_add(int a, int b);
extern void bench_go_add();
*/
import "C"
import (
	"fmt"
	"time"
)

func main() {
	const numCalls = 100_000

	// 测试Go调用Rust
	start := time.Now()
	for i := 0; i < numCalls; i++ {
		_ = C.rust_add(C.int(i), 20)

	}
	elapsed := time.Since(start)
	fmt.Printf("主线程调用: Go -> Rust 调用次数: %d, 平均耗时: %v\n", numCalls, elapsed/time.Duration(numCalls))
	C.bench_go_add()
}

// 初始化Go运行时（空函数）
//
//export InitGoRuntime
func InitGoRuntime() {}

//export GoAdd
func GoAdd(a C.int, b C.int) C.int {
	return a + b
}
