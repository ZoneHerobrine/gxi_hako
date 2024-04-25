use image::{
    error::ImageError, error::ParameterError, error::ParameterErrorKind, ImageBuffer, Luma,
};
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::mem::size_of;
use std::path::Path;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

mod gx;
use gx::gx_const::*;
use gx::gx_enum::*;
use gx::gx_handle::*;
use gx::gx_interface::*;
use gx::gx_pixel_format::*;
use gx::gx_struct::*;

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
        // let gx = GXInterface::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
        let gx = GXInterface::new("GxIAPI.dll").expect("Failed to load library");
        gx.gx_init_lib().expect("Failed to initialize library");

        let mut device_num = 0;
        gx.gx_update_device_list(&mut device_num, 1000)
            .expect("Failed to update device list");

        if device_num > 0 {
            let mut base_info = vec![
                GX_DEVICE_BASE_INFO {
                    szVendorName: [0; GX_INFO_LENGTH_32_BYTE],
                    szModelName: [0; GX_INFO_LENGTH_32_BYTE],
                    szSN: [0; GX_INFO_LENGTH_32_BYTE],
                    szDisplayName: [0; GX_INFO_LENGTH_128_BYTE],
                    szDeviceID: [0; GX_INFO_LENGTH_64_BYTE],
                    szUserID: [0; GX_INFO_LENGTH_64_BYTE],
                    accessStatus: GX_ACCESS_STATUS_CMD::Unknown,
                    deviceClass: GX_DEVICE_CLASS::Unknown,
                    reserved: [0; 300],
                };
                device_num as usize
            ];
            let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
            let status = gx
                .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)
                .expect("Failed to get all device base info");

            if status == 0 {
                // Assuming 0 is GX_STATUS_SUCCESS
                println!(
                    "Device base info retrieved successfully. Number of devices: {}",
                    device_num
                );
                for device in &base_info {
                    print_device_info(&device);
                }

                // Attempt to open the first device using its SN
                let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
                let sn_c_str = CString::new(first_device_sn.trim_end_matches(char::from(0)))
                    .expect("CString::new failed");
                let sn_ptr = sn_c_str.as_ptr() as *const u8;
                let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();
                let open_status = gx
                    .gx_open_device_by_index(1, &mut device_handle)
                    .expect("Failed to open device with SN");

                if open_status == 0 {
                    println!(
                        "Successfully opened device with SN: {}",
                        first_device_sn.trim_end_matches(char::from(0))
                    );

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                        .expect("Failed to send command");

                    // Getting an image
                    // 在这里写获取图像的代码
                    let pixel_size = 2; // BayerRg8、Mono8 格式下每像素1字节
                    let image_size = 2048 * 1536 * pixel_size; // 图像宽*高*每像素字节数
                    let mut image_buffer = vec![1u8; image_size]; // 分配图像缓冲区
                    let mut frame_data = GX_FRAME_DATA {
                        nStatus: GX_FRAME_STATUS::GX_FRAME_STATUS_SUCCESS,
                        pImgBuf: image_buffer.as_mut_ptr() as *mut u8, // 设置图像数据指针
                        nWidth: 2048,
                        nHeight: 1536,
                        // nPixelFormat: PixelFormatEntry::Mono8 as i32,
                        nPixelFormat: 17825795,
                        nImgSize: image_size as i32,
                        nFrameID: 0,
                        nTimestamp: 0,
                        nOffsetX: 0,
                        nOffsetY: 0,
                        reserved: [3],
                    };

                    println!("Attempting to capture image...");
                    println!("Device Handle: {:?}", device_handle);
                    println!("Frame Data Pointer: {:?}", frame_data);

                    let mut width_value: i64 = 1;
                    let get_int_result =
                        gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_WIDTH, &mut width_value);
                    match get_int_result {
                        Ok(_) => println!("Width: {}", width_value),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }
                    let mut height_value: i64 = 1;
                    let get_int_result = gx.gx_get_int(
                        device_handle,
                        GX_FEATURE_ID::GX_INT_HEIGHT,
                        &mut height_value,
                    );
                    match get_int_result {
                        Ok(_) => println!("Height: {}", height_value),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }

                    let mut payload_size: i64 = 1;
                    let get_int_result = gx.gx_get_int(
                        device_handle,
                        GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE,
                        &mut payload_size,
                    );
                    match get_int_result {
                        Ok(_) => println!("Payload Size: {}", payload_size),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }

                    let mut pixel_format: i64 = 1;
                    let get_enum_result = gx.gx_get_enum(
                        device_handle,
                        GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT,
                        &mut pixel_format,
                    );
                    match get_enum_result {
                        Ok(_) => println!("Pixel Format: {}", pixel_format),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }

                    sleep(std::time::Duration::from_secs(1));

                    loop {
                        let result = gx.gx_get_image(device_handle, &mut frame_data, 100);
                        match result {
                            Ok(_) => {
                                println!("Image captured successfully.");
                                break;
                            }
                            Err(e) => eprintln!("Failed to capture image: {:?}", e),
                        }
                        sleep(Duration::from_millis(100));
                    }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                        .expect("Failed to send command");
                    // Processing the image

                    // Close the device
                    gx.gx_close_device(device_handle)
                        .expect("Failed to close device");
                    println!("Device closed.")
                } else {
                    println!(
                        "Failed to open device with SN: {}",
                        first_device_sn.trim_end_matches(char::from(0))
                    );
                }
            } else {
                println!("Failed to retrieve device base info, status: {}", status);
            }
        } else {
            println!("No Devices found.");
        }

        gx.gx_close_lib().expect("Failed to close library");
        println!("Library closed.")
    }
}
