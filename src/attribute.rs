extern crate libc;

use libc::{c_int, c_void};
use ::message::{NetlinkMessage, NetlinkData, nl_msg};


#[link(name="nl-3")]
extern "C" {
    fn nla_put(msg: *const nl_msg, attrtype: c_int, datalen: c_int, data: *const c_void) -> i32;
}


pub enum Type {
    Unspec,
    U8,
    U16,
    U32,
    U64,
    String,
    Flag,
    Msecs,
    Nested,
    __Max
}

pub fn put<T>(msg: &mut NetlinkMessage, atype: i32, len: u32, data: &NetlinkData<T>) -> i32 {
    let vptr = match data.to_vptr() {
        None => return -1,
        Some(ptr) => ptr
    };
    unsafe{ nla_put(::message::expose::nl_msg_ptr(msg), atype as c_int, len as c_int, vptr) }
}

#[macro_export]
macro_rules! NlaPutU8 {
        ($msg:expr, $atype: expr, $data:expr) => {
            rsnl::attribute::put($msg, $atype, std::mem::size_of::<u8>() as u32, $data)
        }
}

#[macro_export]
macro_rules! NlaPutU16 {
        ($msg:expr, $atype: expr, $data:expr) => {
            rsnl::attribute::put($msg, $atype, std::mem::size_of::<u16>() as u32, $data)
        }
}

#[macro_export]
macro_rules! NlaPutU32 {
        ($msg:expr, $atype: expr, $data:expr) => {
            rsnl::attribute::put($msg, $atype, std::mem::size_of::<u32>() as u32, $data)
        }
}

#[macro_export]
macro_rules! NlaPutU64 {
        ($msg:expr, $atype: expr, $data:expr) => {
            rsnl::attribute::put($msg, $atype, std::mem::size_of::<u64>() as u32, $data)
        }
}

#[macro_export]
macro_rules! NlaPutMsec {
        ($msg:expr, $atype: expr, $msecs:expr) => {
            rsnl::attribute::put($msg, $atype, std::mem::size_of::<u64>() as u32, $msecs)
        }
}

// TODO:
// NlaPutAddr!()
// NlaPutData!()
// NlaPutFlag!()
// NlaPutString!()