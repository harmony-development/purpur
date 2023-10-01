use std::{ffi::{CString, c_char}, ptr::null};

use libpurpur::{Purpur, Update, protocols::{Protocol, discord::DiscordProtocol, matrix::MatrixProtocol, irc::IRCProtocol}};

pub struct PurpurC {
    inner: Purpur,
    current_string: CString,
}

#[no_mangle]
pub extern "C" fn purpur_new() -> *mut PurpurC {
    let boxed = Box::new(PurpurC {
        inner: Purpur::new(),
        current_string: CString::default(),
    });
    return Box::into_raw(boxed);
}

#[no_mangle]
pub extern "C" fn purpur_free(ptr: *mut PurpurC) {
    unsafe { drop(Box::from_raw(ptr)); }
}

#[tokio::main]
pub async fn receive(purpurc: &PurpurC) -> Option<Update> {
    purpurc.inner.receive().await
}

#[no_mangle]
pub extern "C" fn purpur_receive(ptr: *mut PurpurC) -> *const c_char {
    let purpurc = unsafe { &mut *ptr };

    match receive(purpurc) {
        Some(update) => {
            let str = serde_json::to_string(&update).expect("serde failed to serialize");
            purpurc.current_string = CString::new(str).expect("invalid cstring generated by serde");
            purpurc.current_string.as_ptr()
        },
        None => null()
    }
}

#[no_mangle]
pub extern "C" fn purpur_add_protocol(ptr: *mut PurpurC, protocol_ptr: *mut Box<dyn Protocol + Send>) {
    let purpurc = unsafe { &mut *ptr };
    let protocol = *unsafe { Box::from_raw(protocol_ptr) };

    purpurc.inner.add_protocol(protocol);
}

#[no_mangle]
pub extern "C" fn purpur_protocol_free(ptr: *mut Box<dyn Protocol + Send>) {
    unsafe { drop(Box::from_raw(ptr)) }
}

#[no_mangle]
pub extern "C" fn purpur_discord_new() -> *mut Box<dyn Protocol + Send> {
    Box::into_raw(Box::new(Box::new(DiscordProtocol::new())))
}

#[no_mangle]
pub extern "C" fn purpur_matrix_new() -> *mut Box<dyn Protocol + Send> {
    Box::into_raw(Box::new(Box::new(MatrixProtocol::new())))
}

#[no_mangle]
pub extern "C" fn purpur_irc_new() -> *mut Box<dyn Protocol + Send> {
    Box::into_raw(Box::new(Box::new(IRCProtocol::new())))
}