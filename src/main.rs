use image::{ImageBuffer, Luma, error::ImageError, error::ParameterErrorKind, error::ParameterError};
use std::path::Path;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::mem::size_of;
use std::slice;
use std::thread::sleep;

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

// Function to save grayscale image data as a PNG file
fn save_grayscale_image(buffer: &[u8], width: u32, height: u32, path: &Path) -> image::ImageResult<()> {
    let img = ImageBuffer::<Luma<u8>, _>::from_raw(width, height, buffer)
        .ok_or(ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)))?;
    img.save(path)
}

// High-level function to handle image capture and saving
fn capture_and_save_image(frame_data: GX_FRAME_DATA) -> image::ImageResult<()> {
    if frame_data.status != 0 {
        return Err(ImageError::Parameter(ParameterError::from_kind(ParameterErrorKind::DimensionMismatch)));
    }

    let width = frame_data.width as u32;
    let height = frame_data.height as u32;

    // Convert the raw pointer back into a Rust slice for safety over the buffer's lifetime
    let buffer_size = frame_data.img_size as usize;
    let buffer_slice = unsafe {
        slice::from_raw_parts(frame_data.p_img_buf as *const u8, buffer_size)
    };

    // Save the image data to a file
    let path = Path::new("output_image.png");
    save_grayscale_image(buffer_slice, width, height, &path)
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

                    let mut frame_data = GX_FRAME_DATA {
                        status: 0,
                        frame_id: 0,
                        p_img_buf: std::ptr::null_mut(),
                        img_size: 3145728,
                        width: 2048,
                        height: 1536,
                        pixel_format: PixelFormatEntry::Mono8 as i32,
                        timestamp: 0,
                        offset_x: 0,
                        offset_y: 0,
                        reserved: [0],
                    };

                    // Allocate buffer for the image
                    let mut buffer = vec![7u8; 2048 * 1536]; // Example size, adjust accordingly
                    frame_data.p_img_buf = buffer.as_mut_ptr() as *mut c_void;
                    frame_data.img_size = (2048 * 1536 * std::mem::size_of::<u8>()) as i32;

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                        .expect("Failed to send command");
                    // Getting an image

                    // Wait for the image to be ready
                    // sleep(std::time::Duration::from_secs(1));

                    gx.gx_get_image(device_handle, &mut frame_data, 1000)
                        .expect("Failed to get image");

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                        .expect("Failed to send command");
                    // Processing the image

                    match capture_and_save_image(frame_data.clone()) {
                        Ok(()) => println!("Image capture and save successful."),
                        Err(e) => eprintln!("Error capturing and saving image: {:?}", e),
                    } // 拿出来还是纯黑的，这个还很要修一段时间

                    println!("Image received. Status: {:?}", frame_data.clone());


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


