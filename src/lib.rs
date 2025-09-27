#![recursion_limit = "16384"]
pub mod client;
pub mod protocol;
pub mod utils;
pub mod handler;
pub mod macros;

const RAKNET_PROTOCOL_VERSION: u8 = 11;
const BEDROCK_PROTOCOL_VERSION: u32 = 827;

/*
use std::ffi::CStr;
use std::os::raw::c_char;
use tokio::runtime::Runtime;
#[no_mangle]
pub extern "C" fn connect_to_server(address: *const c_char, port: u16, version: *const c_char, debug_mode: bool) {
    let c_str_address = unsafe { CStr::from_ptr(address).to_str().unwrap().to_string() };
    let c_str_version = unsafe { CStr::from_ptr(version).to_str().unwrap().to_string() };


    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let client = client::create(c_str_address, port, c_str_version, debug_mode, |code, url| {
            // If you turn on debug, the login code and url will already appear in the console, but you can use this if you want to edit it yourself.
            println!("You can log in with the code {} at {}", code, url);
        }
        );

        client.await.unwrap().connect().expect("Connection Error!");
    });
}


extern "C" {
    fn process_in_cpp(data: *const u8, length: usize);
}

pub fn handle_incoming_data(data: Vec<u8>) {
    let length = data.len();
    unsafe {
        process_in_cpp(data.as_ptr(), length);
    }
}
*/