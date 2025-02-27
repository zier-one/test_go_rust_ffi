use std::{thread, time::Instant};

extern "C" {
    fn InitGoRuntime();
    fn GoAdd(a: libc::c_int, b: libc::c_int) -> libc::c_int;
}

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn bench_go_add() {
    println!("主线程调用:");
    bench_go_add_impl();

    let join_handle = thread::spawn(|| {
        println!("新线程调用，不预热:");
        bench_go_add_impl();
    });
    join_handle.join().unwrap();

    let join_handle = thread::spawn(|| {
        println!("新线程调用，预热:");
        unsafe {
            InitGoRuntime(); // 初始化Go运行时
        }
        bench_go_add_impl();
    });
    join_handle.join().unwrap();
}

fn bench_go_add_impl() {
    let num_calls = 100_000;
    let start = Instant::now();

    for i in 0..num_calls {
        unsafe {
            GoAdd(i, 20);
        }
    }

    let elapsed = start.elapsed();
    println!(
        "Rust -> Go 调用次数: {}, 平均耗时: {:?}",
        num_calls,
        elapsed / num_calls as u32
    );
}
