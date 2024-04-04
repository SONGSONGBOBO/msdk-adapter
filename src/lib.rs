use std::time::Duration;
use libc::c_char;
use anyhow::{Error, Result};

#[link(name = "msdk", kind = "static")]
extern "C" {
    fn M_Open(arg: u64) -> u64;
    fn M_KeyPress(hdl: u64, arg1: i64, arg2: i64) -> i32;
    fn M_KeyInputString(hdl: u64, buf: *const c_char, len: u64) -> i64;
    fn M_GetCurrMousePos2(x: *mut i64, y: *mut i64) -> i64;
    fn M_Close(hdl: u64) -> i32;
}

pub enum MsdkResultCode {
    SUCCESS(i32)
}

impl MsdkResultCode {
    pub fn is_success(code: i32) -> bool {
        return code == 0;
    }
}

const SUCCESS: i32 = 0; 

pub struct MsdkAdapter {
    port: u64,
    handler: u64
}

impl MsdkAdapter {
    // port num, start from 1
    pub fn new(port: u64) -> Result<MsdkAdapter> {
        unsafe {
            let res = MsdkAdapter {
                port,
                handler: M_Open(port)
            };
            std::thread::sleep(Duration::from_secs(3));
            Ok(res)
        }
    }
}

fn check_result(result: i32) -> Result<i32> {
    match result {
        0 => Ok(result),
        _ => {
            Err(Error::msg(format!("Check res failed, result: {}.", result)))
        }
    }
}

pub trait MsdkOperation {
    fn open(&mut self, port_num: u64) -> Result<u64>;
    fn close(&self) -> Result<i32>;
    fn key_press(&self, key_code: i64, count: i64) -> Result<i64>;
}

impl MsdkOperation for MsdkAdapter {
    fn open(&mut self, port_num: u64) -> Result<u64> {
        unsafe {
            self.close().expect(format!("Open port{} fail.", port_num));
            let handler = M_Open(self.port);
            self.handler = handler;
            self.port = port_num;
            Ok(handler)
        }
    }

    fn close(&self) -> Result<i32> {
        unsafe {
            let res = M_Close(self.handler);
            check_result(res)
        }
    }

    fn key_press(&self, key_code: i64, count: i64) -> Result<i64> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime};
    use super::*;

    #[test]
    fn it_works() {
        println!("start{:?}",SystemTime::now());
        let msdk = MsdkAdapter::new(1).unwrap();
        println!("{:?}--{:?}", msdk.handler, SystemTime::now());
        let res = msdk.close().expect("TODO: panic message");
        println!("{:?}", res)
    }
}
