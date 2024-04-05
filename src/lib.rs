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

fn check_result(result: i32) -> Result<i32> {
    match result {
        SUCCESS => Ok(result),
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
            self.close().expect(&format!("Open port{} fail.", port_num));
            let open_func: Symbol<extern "C" fn(u64) -> u64> = self.lib.get(b"M_Open")?;
            let handler = open_func(self.port);
            self.handler = handler;
            self.port = port_num;
            Ok(handler)
        }
    }

    fn close(&self) -> Result<i32> {
        unsafe {
            let func: Symbol<extern "C" fn(u64) -> i32> = self.lib.get(b"M_Close").unwrap();
            let res = func(self.handler);
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
