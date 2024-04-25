// 我想要在接下来已经实现的代码的基础上，进一步开发上述的C例程之中的功能，请你帮助我，谢谢！给出开发好的代码
use image::{
    error::ImageError, error::ParameterError, error::ParameterErrorKind, ImageBuffer, Luma,
};
use opencv::{
    prelude::*,
    highgui,
    imgcodecs,
    core,
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

extern "C" fn frame_callback(p_frame_data: *mut GX_FRAME_CALLBACK_PARAM) {
    println!("Frame callback triggered.");
    println!("Frame status: {:?}", unsafe { (*p_frame_data).status });
    println!("Frame All: {:?}", unsafe { *p_frame_data });
    unsafe {
        let frame_data = &*p_frame_data;
        if frame_data.status == 0 {
            let data = slice::from_raw_parts(frame_data.pImgBuf as *const u8, (frame_data.nWidth * frame_data.nHeight) as usize);
            
            // 使用正确的函数签名创建Mat对象
            let mat = core::Mat::new_rows_cols_with_data(
                frame_data.nHeight, 
                frame_data.nWidth, 
                // core::CV_8UC1, 
                data
            ).unwrap();

            highgui::imshow("Camera Frame", &mat).unwrap();
            if highgui::wait_key(10).unwrap() > 0 {
                highgui::destroy_window("Camera Frame").unwrap();
            }
        }
    }
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
        let gx = GXInterface::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
        // let gx = GXInterface::new("GxIAPI.dll").expect("Failed to load library");
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

                    // let reg_result = gx.gx_register_capture_callback(device_handle, frame_callback);
                    // match reg_result {
                    //     Ok(_) => println!("Capture callback registered successfully."),
                    //     Err(e) => eprintln!("Failed to register capture callback: {:?}", e),
                    // }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                        .expect("Failed to send command");

                    // Getting an image
                    // 在这里写获取图像的代码

                    let pixel_size = 1; // BayerRg8、Mono8 格式下每像素1字节
                    let image_size = 2048 * 1536 * pixel_size; // 图像宽*高*每像素字节数
                    let mut image_buffer = vec![1u8; image_size]; // 分配图像缓冲区
                    let mut frame_data = GX_FRAME_DATA {
                        nStatus: 0,
                        pImgBuf: image_buffer.as_mut_ptr() as *mut c_void, // 设置图像数据指针
                        nWidth: 2048,
                        nHeight: 1536,
                        // nPixelFormat: PixelFormatEntry::Mono8 as i32,
                        nPixelFormat: 17301505,
                        nImgSize: image_size as i32,
                        nFrameID: 0,
                        nTimestamp: 0,
                        reserved: [17301505],
                    };

                        let result = gx.gx_get_image(device_handle, &mut frame_data, 100);
                        match result {
                            Ok(_) => {
                                println!("Image captured successfully.");
                            }
                            Err(e) => eprintln!("Failed to capture image: {:?}", e),
                        }

                        
                    // highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
                    // loop {
                    //     sleep(Duration::from_secs(10));
                    //     break;
                    // }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                        .expect("Failed to send command");
                    // Processing the image

                    // let unregeister_result = gx.gx_unregister_capture_callback(device_handle);
                    // match unregeister_result {
                    //     Ok(_) => println!("Capture callback unregistered successfully."),
                    //     Err(e) => eprintln!("Failed to unregister capture callback: {:?}", e),
                    // }

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
