open failed but init sucess
```rust
use libloading::{Library, Symbol};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

// 定义函数的类型签名
type GXInitLib = unsafe extern "C" fn() -> i32;
type GXCloseLib = unsafe extern "C" fn() -> i32;
type GXUpdateDeviceList = unsafe extern "C" fn(device_num: *mut u32, timeout: u32) -> i32;
type GXGetAllDeviceBaseInfo = unsafe extern "C" fn(base_info: *mut c_void, size: *mut u32) -> i32;
type GXOpenDevice = unsafe extern "C" fn(open_param: *const c_void, device_handle: *mut *mut c_void) -> i32;
type GXCloseDevice = unsafe extern "C" fn(device_handle: *mut c_void) -> i32;
type GXGetDeviceIPInfo = unsafe extern "C" fn(device_id: u32, ip_info: *mut c_void) -> i32;
type GXSetDevicePersistentIpAddress = unsafe extern "C" fn(device_handle: *mut c_void, ip_address: *const c_char, subnet_mask: *const c_char, default_gateway: *const c_char) -> i32;
type GXGigEIpConfiguration = unsafe extern "C" fn(mac_address: *const c_char, ip_config_mode: i32, ip_address: *const c_char, subnet_mask: *const c_char, default_gateway: *const c_char, user_id: *const c_char) -> i32;



#[repr(C)]
struct GX_OPEN_PARAM {
    access_mode: u32,
    open_mode: u32,
    psz_content: *const c_char,
}

fn main() {
    unsafe {
        let lib = Library::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");

        let gx_init_lib: Symbol<GXInitLib> = lib.get(b"GXInitLib").expect("Failed to load GXInitLib");
        let gx_close_lib: Symbol<GXCloseLib> = lib.get(b"GXCloseLib").expect("Failed to load GXCloseLib");
        let gx_update_device_list: Symbol<GXUpdateDeviceList> = lib.get(b"GXUpdateDeviceList").expect("Failed to load GXUpdateDeviceList");
        let gx_open_device: Symbol<GXOpenDevice> = lib.get(b"GXOpenDevice").expect("Failed to load GXOpenDevice");
        let gx_close_device: Symbol<GXCloseDevice> = lib.get(b"GXCloseDevice").expect("Failed to load GXCloseDevice");

        // 初始化 GxIAPI
        gx_init_lib();

        // 枚举设备
        let mut device_num = 0;
        gx_update_device_list(&mut device_num, 1000);
        println!("Number of devices: {}", device_num);

        if device_num > 0 {
            let open_param_sn = CString::new("FE0180090084").unwrap();  // Bind the CString to a variable
            let open_param = GX_OPEN_PARAM {
                access_mode: 0,  // GX_ACCESS_EXCLUSIVE
                open_mode: 1,    // GX_OPEN_SN
                psz_content: open_param_sn.as_ptr(),  // Use the CString variable
            };

            println!("Opening device with SN: {:?}", open_param_sn);

            let mut device_handle: *mut c_void = std::ptr::null_mut();
            let result = gx_open_device(&open_param as *const _ as *const c_void, &mut device_handle);

            if result != 0 { // Assuming 0 is success
                println!("Failed to open device, error code: {}", result);
            } else {
                println!("Device opened successfully.");
            }

            // 关闭设备
            gx_close_device(device_handle);
        }

        // 反初始化 GxIAPI
        gx_close_lib();
    }
}

```