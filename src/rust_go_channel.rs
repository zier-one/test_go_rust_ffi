#[no_mangle]
pub extern "C" fn rust_add_async(a: i32, b: i32, done_chan: *const std::ffi::c_void) {
    // let run = async move {
    let result = a + b;
    println!("Rust add result: {} {:?}", result, done_chan);
    // };
}
