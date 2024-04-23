```rust
use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};

// 定义函数的类型签名
type GXOpenDevice = unsafe extern "C" fn(device_name: *const c_char, ph_device: *mut *mut c_void) -> i32;
type GXCloseDevice = unsafe extern "C" fn(h_device: *mut c_void) -> i32;

fn main() {
    unsafe {
        // 加载 DLL
        let lib = Library::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");

        // 加载函数
        let gx_open_device: Symbol<GXOpenDevice> = lib.get(b"GXOpenDevice").expect("Failed to load function");
        let gx_close_device: Symbol<GXCloseDevice> = lib.get(b"GXCloseDevice").expect("Failed to load function");

        // 调用函数
        let mut h_device: *mut c_void = std::ptr::null_mut();
        let result = gx_open_device(CStr::from_bytes_with_nul_unchecked(b"DeviceName\0").as_ptr(), &mut h_device);
        println!("GXOpenDevice result: {}", result);

        // 确保设备已经打开再关闭
        if !h_device.is_null() {
            let close_result = gx_close_device(h_device);
            println!("GXCloseDevice result: {}", close_result);
        }
    }
}

```