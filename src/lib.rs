#![recursion_limit = "16384"]
pub mod client;
pub mod handler;
pub mod protocol;
pub mod utils;

pub const RAKNET_PROTOCOL_VERSION: u8 = 11;
pub const BEDROCK_PROTOCOL_VERSION: u32 = 1001;
const VANILLA_BLOCK_PALETTE: &[u8] = include_bytes!("../resources/block_palette_1001.nbt");

/*use std::ffi::{c_char, CStr, CString};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use crate::client::Client;

type PacketCallback = extern "C" fn(*const c_char, *const c_char);
pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

#[no_mangle]
pub extern "C" fn connect_to_server(
    address: *const c_char,
    port: u16,
    version: *const c_char,
    callback: PacketCallback) -> *mut Client {
    let addr = unsafe { CStr::from_ptr(address).to_str().unwrap().to_string() };
    let ver = unsafe { CStr::from_ptr(version).to_str().unwrap().to_string() };

    let client = RUNTIME.block_on(async move {
        client::create(addr, port, ver, false, |_, _| {}).await.unwrap()
    });

    let boxed_client = Box::new(client);
    let raw_ptr = Box::into_raw(boxed_client);

    let client_ptr = raw_ptr;

    let ptr_addr = client_ptr as usize;

    RUNTIME.spawn(async move {
        let client = unsafe { &mut *(ptr_addr as *mut Client) };

        loop {
            match client.next_event().await {
                Some((name, packet)) => {
                    let packet_json = serde_json::to_string(&packet).unwrap_or_else(|_| "Error".to_string());
                    let name_string = CString::new(name.clone()).unwrap();
                    let c_string = CString::new(packet_json).unwrap();
                    callback(name_string.as_ptr(), c_string.as_ptr());
                },
                None => {
                    continue;
                }
            }
        }
    });

    raw_ptr
}

#[no_mangle]
pub extern "C" fn send_packet_to_server(client: *mut Client, data: *const u8, len: usize) {
    if client.is_null() { return; }
    let client = unsafe { &mut *client };
    let vec_data = unsafe { std::slice::from_raw_parts(data, len).to_vec() };
    client.send_packet(vec_data);
}

#[no_mangle]
pub extern "C" fn disconnect_client(client: *mut Client) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}*/