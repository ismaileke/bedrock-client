#![recursion_limit = "16384"]
pub mod client;
pub mod protocol;
pub mod utils;
/*use std::ffi::CString;
use std::os::raw::c_char;
use tokio::runtime::Runtime;

#[no_mangle]
pub extern "C" fn connect_to_server(address: *const c_char, port: u16, version: *const c_char, debug_mode: bool) {
    let c_str_address = unsafe { CString::from_raw(address as *mut c_char) };
    let c_str_version = unsafe { CString::from_raw(version as *mut c_char) };

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = client::create(
            c_str_address.to_str().unwrap().to_string(),
            port,
            c_str_version.to_str().unwrap().to_string(),
            debug_mode,
        );

        client.await.unwrap().connect().expect("Connection Error!");
    });
}*/
