use libloading::{Library, Symbol};
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
}这是我已有的rust代码，Device base info retrieved successfully. Number of devices: 1
Vendor Name: Daheng Imaging
Model Name: MER-302-56U3M
Serial Number: FE0180090084
Display Name: MER-302-56U3M(FE0180090084)
Device ID: MER-302-56U3M(FE0180090084)
User ID:
Access Status: Unknown
Device Class: U3v
-----------------------这个是输出，我希望在这个基础上进一步开发SN连接相机,谢谢！

---

    pub unsafe fn gx_get_image(&self, device: GX_DEV_HANDLE, frame_data: &mut GX_FRAME_DATA, timeout: i32) -> Result<i32, CameraError> {
        let gx_get_image: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE, frame_data: *mut GX_FRAME_DATA, timeout: i32) -> i32> = self.lib.get(b"GXGetImage")?;
        let status = gx_get_image(device, frame_data as *mut _, timeout);
        if status == GX_STATUS_LIST::GX_STATUS_SUCCESS as i32 {
            Ok(status)
        } else {
            // Err(libloading::Error::from(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get image: Status {}", status))))
            // Err(format!("Failed to get image: Status {}", status))
            Err(CameraError::OperationError(format!("Failed to get image: Status {}", status)))
        }
    },结构体是
    #[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FRAME_DATA {
    pub status: u32,            // Image acquisition status
    pub frame_id: u64,          // Frame ID
    pub p_img_buf: *mut u8,     // Pointer to the image buffer
    pub img_size: usize,        // Size of the image buffer
    pub width: u32,             // Image width
    pub height: u32,            // Image height
    pub pixel_format: u32,      // Pixel format
    pub timestamp: u64,         // Timestamp of the frame
}
    
    这个我在这
            let mut frame_data = GX_FRAME_DATA {
            status: 0,
            frame_id: 0,
            p_img_buf: std::ptr::null_mut(),
            img_size: 0,
            width: 2048,
            height: 1536,
            pixel_format: 0,
            timestamp: 0,
        };
            let payload_size = 4096 * 3000; // Adjust size accordingly
        let mut buffer = vec![0u8; payload_size];
        frame_data.p_img_buf = buffer.as_mut_ptr() as *mut u8;
        frame_data.img_size = payload_size;        gx.gx_get_image(device_handle, &mut frame_data, 1000).expect("Failed to get image");
调用的时候报错
Failed to get image: OperationError("Failed to get image: Status -5")，也就是GX_STATUS_INVALID_PARAMETER 用户传入图像地址指针为NULL，请你帮我解决一下，谢谢！