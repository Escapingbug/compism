use compism::Compism;
use std::sync::Mutex;
use std::path::Path;

lazy_static! {
    static ref COMPISM: Mutex<Compism> = Mutex::new(Compism::new());
}

#[no_mangle]
pub extern fn compism_open(path: &Path) -> i32 {
    match COMPISM.lock() {
        Err(_) => {
            return -1;
        }
        Ok(ref mut c) => {
            match c.find_route_mut(path) {
                Err(_) => return -1,
                Ok(ref r) => {
                    match r.plugin {
                        None => return -1,
                        Some(ref p) => {
                            unsafe {
                                (*p.operations.open)()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern fn compism_close(path: &Path) -> i32 {
    match COMPISM.lock() {
        Err(_) => {
            return -1;
        }
        Ok(ref mut c) => {
            match c.find_route_mut(path) {
                Err(_) => return -1,
                Ok(ref r) => {
                    match r.plugin {
                        None => return -1,
                        Some(ref p) => {
                            unsafe {
                                (*p.operations.close)()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern fn compism_llseek(path: &Path, off: i32, whence: i32) -> i32 {
    match COMPISM.lock() {
        Err(_) => {
            return -1;
        }
        Ok(ref mut c) => {
            match c.find_route_mut(path) {
                Err(_) => return -1,
                Ok(ref r) => {
                    match r.plugin {
                        None => return -1,
                        Some(ref p) => {
                            unsafe {
                                (*p.operations.llseek)(off, whence)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub extern fn compism_read(path: &Path, buf: *mut u8, size: i32) -> i32 {
    match COMPISM.lock() {
        Err(_) => {
            return -1;
        }
        Ok(ref mut c) => {
            match c.find_route_mut(path) {
                Err(_) => return -1,
                Ok(ref r) => {
                    match r.plugin {
                        None => return -1,
                        Some(ref p) => {
                            unsafe {
                                (*p.operations.read)(buf, size)
                            }
                        }
                    }
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn compism_write(path: &Path, buf: *const u8, size: i32) -> i32 {
    match COMPISM.lock() {
        Err(_) => {
            return -1;
        }
        Ok(ref mut c) => {
            match c.find_route_mut(path) {
                Err(_) => return -1,
                Ok(ref r) => {
                    match r.plugin {
                        None => return -1,
                        Some(ref p) => {
                            (*p.operations.write)(buf, size)
                        }
                    }
                }
            }
        }
    }
}

/*
#[no_mangle]
pub unsafe extern "C" fn compism_ioctl(path: &Path, cmd: i32, mut args: ...) -> i32 {
    unimplemented!()
}
*/
