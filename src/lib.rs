use std::time::Duration;
use anyhow::{Error, Result};
use libloading::{Library, Symbol};

const SUCCESS: i32 = 0; 

pub struct MsdkAdapter {
    port: u64,
    handler: u64,
    lib: Library
}

impl MsdkAdapter {
    // port num, start from 1
    pub fn new(port: u64) -> Result<MsdkAdapter> {
        unsafe {
            let lib = Library::new("msdk.dll")?;
            let open_func: Symbol<extern "C" fn(u64) -> u64> = lib.get(b"M_Open")?;
            let res = MsdkAdapter {
                port,
                handler: open_func(port),
                lib
            };
            std::thread::sleep(Duration::from_secs(3));
            Ok(res)
        }
    }
}

pub trait MsdkKeyBoardOperation {
    fn open(&mut self, port_num: u64) -> Result<u64>;
    fn close(&self) -> Result<i32>;
    // windows keycode ascii
    fn key_press(&self, key_code: i32, count: i32) -> Result<i32>;
    fn key_down(&self, key_code: i32) -> Result<i32>;
    fn key_up(&self, key_code: i32) -> Result<i32>;
    fn all_key_up(&self) -> Result<i32>;
}

pub trait MsdkMouseOperation {
    fn left_click(&self, count: i32) -> Result<i32>;
    fn left_double_click(&self, count: i32) -> Result<i32>;
}

fn check_result(result: i32) -> Result<i32> {
    match result {
        SUCCESS => Ok(result),
        _ => {
            Err(Error::msg(format!("Check res failed, result: {}.", result)))
        }
    }
}

impl MsdkKeyBoardOperation for MsdkAdapter {
    fn open(&mut self, port_num: u64) -> Result<u64> {
        unsafe {
            self.close().expect(&format!("Open port{} fail.", port_num));
            let func: Symbol<extern "C" fn(u64) -> u64> = self.lib.get(b"M_Open")?;
            let handler = func(self.port);
            self.handler = handler;
            self.port = port_num;
            Ok(handler)
        }
    }

    fn close(&self) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64) -> i32> = self.lib.get(b"M_Close")?;
            let res = func(self.handler);
            check_result(res)
        }
    }

    fn key_press(&self, key_code: i32, count: i32) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64, i32, i32) -> i32> = self.lib.get(b"M_KeyPress2")?;
            let res = func(self.handler, key_code, count);
            check_result(res)
        }
    }

    fn key_down(&self, key_code: i32) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64, i32) -> i32> = self.lib.get(b"M_KeyDown2")?;
            let res = func(self.handler, key_code);
            check_result(res)
        }
    }

    fn key_up(&self, key_code: i32) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64, i32) -> i32> = self.lib.get(b"M_KeyUp2")?;
            let res = func(self.handler, key_code);
            check_result(res)
        }
    }

    fn all_key_up(&self) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64) -> i32> = self.lib.get(b"M_ReleaseAllKey")?;
            let res = func(self.handler);
            check_result(res)
        }
    }
}
