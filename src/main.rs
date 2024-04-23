use std::mem::size_of;
use std::ffi::{CStr, CString,c_char,c_uint,c_int,c_void};

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
        let gx = GXInterface::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
        gx.gx_init_lib().expect("Failed to initialize library");

        let mut device_num = 0;
        gx.gx_update_device_list(&mut device_num, 1000).expect("Failed to update device list");

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
            let status = gx.gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size).expect("Failed to get all device base info");

            if status == 0 { // Assuming 0 is GX_STATUS_SUCCESS
                println!("Device base info retrieved successfully. Number of devices: {}", device_num);
                for device in &base_info {
                    print_device_info(&device);
                }
                
                // Attempt to open the first device using its SN
                let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
                let sn_c_str = CString::new(first_device_sn.trim_end_matches(char::from(0))).expect("CString::new failed");
                let sn_ptr = sn_c_str.as_ptr() as *const u8;
                let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();
                let open_status = gx.gx_open_device_by_index(1, &mut device_handle).expect("Failed to open device with SN");

                if open_status == 0 {
                    println!("Successfully opened device with SN: {}", first_device_sn.trim_end_matches(char::from(0)));
                    // Further operations can be performed here...


                    // Assume device handle and other preparation steps are already done
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

        // You need to ensure the buffer is correctly allocated according to the expected image size
        // Example for allocation:
        let payload_size = 4096 * 3000; // Adjust size accordingly
        let mut buffer = vec![7u8; payload_size];
        frame_data.p_img_buf = buffer.as_mut_ptr() as *mut c_void;
        frame_data.img_size = payload_size;

        gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START).expect("Failed to send command");
        // Getting an image
        gx.gx_get_image(device_handle, &mut frame_data, 1000).expect("Failed to get image");

        gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP).expect("Failed to send command");
        // Processing the image
        // For example, accessing the buffer: &buffer[..]
        let buffer = unsafe { Vec::from_raw_parts(frame_data.p_img_buf, payload_size, payload_size) };


                    // Close the device
                    gx.gx_close_device(device_handle).expect("Failed to close device");
                    println!("Device closed.")
                } else {
                    println!("Failed to open device with SN: {}", first_device_sn.trim_end_matches(char::from(0)));
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