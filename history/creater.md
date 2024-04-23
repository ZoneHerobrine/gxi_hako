也就是抑制文档里面的结构体、枚举还有函数到rust里面，用libloading这个库，进行开发，use libloading::{Library, Symbol};
use std::mem::size_of;


const GX_INFO_LENGTH_32_BYTE: usize = 32;
const GX_INFO_LENGTH_128_BYTE: usize = 128 + 4;
const GX_INFO_LENGTH_64_BYTE: usize = 64 + 4;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
enum GX_DEVICE_CLASS {
    Unknown = 0,
    Usb2 = 1,
    Gev = 2,
    U3v = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
enum GX_ACCESS_STATUS_CMD {
    Unknown = 0,
    ReadWrite = 1,
    ReadOnly = 2,
    NoAccess = 3,
}


#[repr(C)]
#[derive(Debug, Clone)]
struct GX_DEVICE_BASE_INFO {
    szVendorName: [u8; GX_INFO_LENGTH_32_BYTE],
    szModelName: [u8; GX_INFO_LENGTH_32_BYTE],
    szSN: [u8; GX_INFO_LENGTH_32_BYTE],
    szDisplayName: [u8; GX_INFO_LENGTH_128_BYTE],
    szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE],
    szUserID: [u8; GX_INFO_LENGTH_64_BYTE],
    accessStatus: GX_ACCESS_STATUS_CMD,
    deviceClass: GX_DEVICE_CLASS,
    reserved: [u8; 300],
}

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
            let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
            let status = gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size);

            if status == 0 { // Assuming 0 is GX_STATUS_SUCCESS
                println!("Device base info retrieved successfully. Number of devices: {}", device_num);
                for device in base_info {
                    print_device_info(&device);
                }
            } else {
                println!("Failed to retrieve device base info, status: {}", status);
            }
        } else {
            println!("No devices found.");
        }

        gx_close_lib();
    }
}例如这就是我写到获取设备信息的一个成功的范例，Device base info retrieved successfully. Number of devices: 1
Vendor Name: Daheng Imaging
Model Name: MER-302-56U3M
Serial Number: FE0180090084
Display Name: MER-302-56U3M(FE0180090084)
Device ID: MER-302-56U3M(FE0180090084)
User ID:
Access Status: Unknown
Device Class: U3v
-----------------------他会拿到这个，接下来我会在这份代码的基础上进一步开发，来开启相机和推流之类的，希望你能提供支持，谢谢！