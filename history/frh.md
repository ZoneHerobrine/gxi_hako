index连接成功，除了interface全部都封装了，准备大改
```rust
use libloading::{Library, Symbol};
use std::mem::size_of;
use std::ffi::{CStr, CString,c_char,c_uint,c_int};

mod gx;

use gx::gx_const::*;
use gx::gx_enum::*;
use gx::gx_struct::*;
use gx::gx_handle::*;
use gx::gx_interface::*;

fn print_device_info(device_info: &GX_DEVICE_BASE_INFO) {
    let vendor_name = std::str::from_utf8(&device_info.szVendorName).unwrap_or("[Invalid UTF-8]");
    let model_name = std::str::from_utf8(&device_info.szModelName).unwrap_or("[Invalid UTF-8]");
    let serial_number = std::str::from_utf8(&device_info.szSN).unwrap_or("[Invalid UTF-8]");
    let display_name = std::str::from_utf8(&device_info.szDisplayName).unwrap_or("[Invalid UTF-8]");
    let device_id = std::str::from_utf8(&device_info.szDeviceID).unwrap_or("[Invalid UTF-8]");
    let user_id = std::str::from_utf8(&device_info.szUserID).unwrap_or("[Invalid UTF-8]");

    println!("Vendor Name: {}", vendor_name);
    println!("Model Name: {}", model_name);
    println!("Serial Number: {}", serial_number);
    println!("Display Name: {}", display_name);
    println!("Device ID: {}", device_id);
    println!("User ID: {}", user_id);
    println!("Access Status: {:?}", device_info.accessStatus);
    println!("Device Class: {:?}", device_info.deviceClass);
    println!("-----------------------");
}


fn main() {
    unsafe {
        let lib = Library::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");

        let gx_init_lib: Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"GXInitLib").expect("Failed to load GXInitLib");
        let gx_update_device_list: Symbol<unsafe extern "C" fn(device_num: *mut u32, timeout: u32) -> i32> = lib.get(b"GXUpdateDeviceList").expect("Failed to load GXUpdateDeviceList");
        let gx_get_all_device_base_info: Symbol<unsafe extern "C" fn(p_device_info: *mut GX_DEVICE_BASE_INFO, p_buffer_size: *mut usize) -> i32> = lib.get(b"GXGetAllDeviceBaseInfo").expect("Failed to load GXGetAllDeviceBaseInfo");
        let gx_close_lib: Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"GXCloseLib").expect("Failed to load GXCloseLib");
        let gx_open_device: Symbol<unsafe extern "C" fn(pszContent: *const u8, device: *mut GX_DEV_HANDLE) -> i32> =
        lib.get(b"GXOpenDevice").expect("Failed to load GXOpenDevice");
    let gx_close_device: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> =
        lib.get(b"GXCloseDevice").expect("Failed to load GXCloseDevice");
        let gx_open_device_by_index: Symbol<unsafe extern "C" fn(index: u32, device: *mut GX_DEV_HANDLE) -> i32> = lib.get(b"GXOpenDeviceByIndex").expect("Failed to load GXOpenDeviceByIndex");
        gx_init_lib();

        let mut device_num = 0;
        gx_update_device_list(&mut device_num, 1000);

        if device_num > 0 {
            let mut base_info = vec![GX_DEVICE_BASE_INFO {
                szVendorName: [0; GX_INFO_LENGTH_32_BYTE],
                szModelName: [0; GX_INFO_LENGTH_32_BYTE],
                szSN: [0; GX_INFO_LENGTH_32_BYTE],
                szDisplayName: [0; GX_INFO_LENGTH_128_BYTE],
                szDeviceID: [0; GX_INFO_LENGTH_64_BYTE],
                szUserID: [0; GX_INFO_LENGTH_64_BYTE],
                accessStatus: GX_ACCESS_STATUS_CMD::Unknown,
                deviceClass: GX_DEVICE_CLASS::Unknown,
                reserved: [0; 300],
            }; device_num as usize];
            // let mut open_param = GX_OPEN_PARAM {
            //     pszContent: std::ptr::null(),
            //     openMode: GX_OPEN_MODE_CMD::GX_OPEN_INDEX,
            //     accessMode: GX_ACCESS_MODE_CMD::EXCLUSIVE,
            // }; // 这里有类型错误说明3.5确实写的有问题，过后用4重写3.5
            let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
            let status = gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size);

            if status == 0 { // Assuming 0 is GX_STATUS_SUCCESS
                println!("Device base info retrieved successfully. Number of devices: {}", device_num);
                for device in &base_info {
                    print_device_info(&device);
                }
                
                // 这里是SN打开的逻辑
                // Attempt to open the first device using its SN
                let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
                // let sn_c_str = CString::new(first_device_sn.trim_end_matches(char::from(0))).expect("CString::new failed");
                // let sn_ptr = sn_c_str.as_ptr() as *const u8; // 类型转换
                // let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();
                // let open_status = gx_open_device(sn_ptr, &mut device_handle);
                // 这里是SN打开的逻辑

                // Index打开成功了，但是SN打开的时候strlen里面的汇编报内存错误了草，先不用了,哦破案了，其实是参数穿的不对，还有其他配置，等有GPT再调了
                let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();
                let open_status = gx_open_device_by_index(1, &mut device_handle);
                // 这里是Index打开的逻辑，不用设置open_param

                if open_status == 0 {
                    println!("Successfully opened device with SN: {}", first_device_sn.trim_end_matches(char::from(0)));
                    // Further operations can be performed here...

                    // Close the device
                    gx_close_device(device_handle);
                    println!("Device closed.")
                } else {
                    println!("Failed to open device with SN: {}", first_device_sn.trim_end_matches(char::from(0)));
                }
            } else {
                println!("Failed to retrieve device base info, status: {}", status);
            }
        } else {
            println!("No devices found.");
        }

        gx_close_lib();
        println!("Library closed.")
    }
}

```