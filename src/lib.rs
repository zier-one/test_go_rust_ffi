use std::{thread, time::Duration};

#[no_mangle]
pub extern "C" fn rust_simple_fn() {
    println!("Rust函数被Go调用");
    unsafe {
        GoSimpleFn();
    }
}

#[no_mangle]
pub extern "C" fn rust_run_callback(func: extern "C" fn(i32)) {
    println!("Rust try to run callback");
    unsafe {
        GoSimpleFn();
        func(3)
    }
}

#[no_mangle]
pub extern "C" fn rust_notify() {
    thread::spawn(|| {
        println!("Rust notify goroutine after 3 seconds");
        thread::sleep(Duration::from_secs(3));
        unsafe {
            GoWakeUpGoroutine();
        }
        println!("Rust notify goroutine");
    });
}


pub fn gettid() -> u64 {
    unsafe { libc::syscall(libc::SYS_gettid) as u64 }
}

#[no_mangle]
pub extern "C" fn rust_print_tid() {
    println!("Rust tid: {}",gettid());
    unsafe {
        GoPrintTID(gettid());
    }
    thread::spawn(|| {
        println!("Rust tid: {}",gettid());
        unsafe {
            GoPrintTID(gettid());
        }
    });
    thread::sleep(Duration::from_secs(1));
    println!("rust_print_tid return");
}

extern "C" {
    fn GoSimpleFn();
    fn GoWakeUpGoroutine();
    fn GoPrintTID(_: u64);
}
