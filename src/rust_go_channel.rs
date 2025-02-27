use tokio::runtime::{Builder, Runtime};

extern "C" {
    fn PushChannel(resp: libc::c_int, doneChPtr: *const std::ffi::c_void);
}
static mut RT: *mut Runtime = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn init_tokio_runtime() {
    let tokio_rt = Builder::new_multi_thread()
        .worker_threads(16)
        .enable_all()
        .build()
        .unwrap();
    tokio_rt.block_on(async {
        println!("Tokio runtime initialized");
    });
    let tokio_rt = Box::new(tokio_rt);
    unsafe {
        RT = Box::into_raw(tokio_rt);
    }
}

#[derive(Clone, Copy)]
struct SendWarper(*const std::ffi::c_void);
unsafe impl Send for SendWarper {}

#[no_mangle]
pub extern "C" fn rust_add_async(a: i32, b: i32, done_chan: *const std::ffi::c_void) {
    let done_chan = SendWarper(done_chan);
    let run = async move {
        let done_chan = done_chan;
        let result = a + b;
        // println!("Rust add result: {} {:?}", result, done_chan.0);
        unsafe {
            PushChannel(result, done_chan.0);
        }
    };
    unsafe { (*RT).spawn(run) };
}
