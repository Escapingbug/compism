use lib;
#[cfg(unix)]
use lib::os::unix::*;
#[cfg(windows)]
use lib::os::windows::*;
use std::path::Path;
use error::Result;

#[derive(Debug)]
pub struct Plugin {
    /// Library loaded of this plugin
    pub library: Box<lib::Library>,
    /// Operations of the plugin
    pub operations: PluginOperations,
}

#[derive(Debug)]
pub struct PluginOperations {
    pub llseek: Symbol<unsafe extern fn(off: i32, whence: i32) -> i32>,
    pub read: Symbol<unsafe extern fn(buf: *mut u8, size: i32) -> i32>,
    pub write: Symbol<unsafe extern fn(buf: *const u8, size: i32) -> i32>,
    pub open: Symbol<unsafe extern fn() -> i32>,
    pub close: Symbol<unsafe extern fn() -> i32>,
    //ioctl: Symbol<unsafe extern fn(cmd: i32, ...)>,
}

impl Plugin {
    /// Load a new plugin in `lib_path`. Lib should be `.dll` or `.so`. Please refer to
    /// documentation to get the specific format of a plugin library file.
    pub fn new(lib_path: &Path) -> Result<Self> {
        // The `rental` crate is so complicated to use, since I can guarantee that
        // the operations' lifetime is along with the library, just go with it using
        // unsafe tricks instead of that complicated rental.
        let lib = Box::new(lib::Library::new(lib_path)?);
        unsafe {
            let op = {
                let llseek: lib::Symbol<unsafe extern fn(i32, i32) -> i32> = lib.get(b"plugin_llseek")?;
                let read: lib::Symbol<unsafe extern fn(*mut u8, i32) -> i32> = lib.get(b"plugin_read")?;
                let write: lib::Symbol<unsafe extern fn(*const u8, i32) -> i32> = lib.get(b"plugin_write")?;
                let open: lib::Symbol<unsafe extern fn() -> i32> = lib.get(b"plugin_open")?;
                let close: lib::Symbol<unsafe extern fn() -> i32> = lib.get(b"plugin_close")?;
                //let ioctl: lib::Symbol<unsafe extern fn(cmd: i32, ...)> = lib.get(b"plugin_ioctl")?;
                let op = PluginOperations {
                    llseek: llseek.into_raw(),
                    read: read.into_raw(),
                    write: write.into_raw(),
                    open: open.into_raw(),
                    close: close.into_raw(),
                    //ioctl: ioctl.into_raw(),
                };
                op
            };
            Ok(Plugin {
                library: lib,
                operations: op
            })
        }
    }

}
