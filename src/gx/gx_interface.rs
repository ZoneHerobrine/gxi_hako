use libloading::{Library, Symbol};
use std::ffi::{CStr, c_char, c_void};
use crate::gx::gx_handle::*;
use crate::gx::gx_struct::*;
use crate::gx::gx_const::*;
use crate::gx::gx_enum::*;

// Define a custom error type
#[derive(Debug)]
pub enum CameraError {
    LibraryError(libloading::Error),
    OperationError(String),
}

impl From<libloading::Error> for CameraError {
    fn from(err: libloading::Error) -> Self {
        CameraError::LibraryError(err)
    }
}

pub struct GXInterface {
    lib: Library,
}

fn convert_to_gx_status(status_code: i32) -> GX_STATUS_LIST {
    match status_code {
        0 => GX_STATUS_LIST::GX_STATUS_SUCCESS,
        -1 => GX_STATUS_LIST::GX_STATUS_ERROR,
        -2 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_TL,
        -3 => GX_STATUS_LIST::GX_STATUS_NOT_FOUND_DEVICE,
        -4 => GX_STATUS_LIST::GX_STATUS_OFFLINE,
        -5 => GX_STATUS_LIST::GX_STATUS_INVALID_PARAMETER,
        -6 => GX_STATUS_LIST::GX_STATUS_INVALID_HANDLE,
        -7 => GX_STATUS_LIST::GX_STATUS_INVALID_CALL,
        -8 => GX_STATUS_LIST::GX_STATUS_INVALID_ACCESS,
        -9 => GX_STATUS_LIST::GX_STATUS_NEED_MORE_BUFFER,
        -10 => GX_STATUS_LIST::GX_STATUS_ERROR_TYPE,
        -11 => GX_STATUS_LIST::GX_STATUS_OUT_OF_RANGE,
        -12 => GX_STATUS_LIST::GX_STATUS_NOT_IMPLEMENTED,
        -13 => GX_STATUS_LIST::GX_STATUS_NOT_INIT_API,
        -14 => GX_STATUS_LIST::GX_STATUS_TIMEOUT,
        _ => GX_STATUS_LIST::GX_STATUS_ERROR, // Default error if unknown status code
    }
}


impl GXInterface {
    pub unsafe fn new(library_path: &str) -> Result<Self, libloading::Error> {
        let lib = Library::new(library_path)?;
        Ok(GXInterface { lib })
    }

    pub unsafe fn gx_init_lib(&self) -> Result<i32, libloading::Error> {
        let gx_init_lib: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"GXInitLib")?;
        Ok(gx_init_lib())
    }

    pub unsafe fn gx_update_device_list(&self, device_num: *mut u32, timeout: u32) -> Result<i32, libloading::Error> {
        let gx_update_device_list: Symbol<unsafe extern "C" fn(device_num: *mut u32, timeout: u32) -> i32> = self.lib.get(b"GXUpdateDeviceList")?;
        Ok(gx_update_device_list(device_num, timeout))
    }

    pub unsafe fn gx_get_all_device_base_info(&self, p_device_info: *mut GX_DEVICE_BASE_INFO, p_buffer_size: *mut usize) -> Result<i32, libloading::Error> {
        let gx_get_all_device_base_info: Symbol<unsafe extern "C" fn(p_device_info: *mut GX_DEVICE_BASE_INFO, p_buffer_size: *mut usize) -> i32> = self.lib.get(b"GXGetAllDeviceBaseInfo")?;
        println!("p_device_info: {:?}, p_buffer_size: {:?}", p_device_info, p_buffer_size);
        Ok(gx_get_all_device_base_info(p_device_info, p_buffer_size))
    }

    pub unsafe fn gx_open_device_by_index(&self, index: u32, device: *mut GX_DEV_HANDLE) -> Result<i32, libloading::Error> {
        let gx_open_device_by_index: Symbol<unsafe extern "C" fn(index: u32, device: *mut GX_DEV_HANDLE) -> i32> = self.lib.get(b"GXOpenDeviceByIndex")?;
        Ok(gx_open_device_by_index(index, device))
    }

    pub unsafe fn gx_send_command(
        &self, 
        device: GX_DEV_HANDLE, 
        feature_id: GX_FEATURE_ID
    ) -> Result<(), CameraError> {
        let gx_send_command: Symbol<unsafe extern "C" fn(GX_DEV_HANDLE, GX_FEATURE_ID) -> i32> = self.lib.get(b"GXSendCommand")?;
        
        let status_code = gx_send_command(device, feature_id);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!("GXSendCommand failed with status: {:?}", status))),
        }
    }
    
    pub unsafe fn gx_get_image(&self, device: GX_DEV_HANDLE, p_frame_data: *mut GX_FRAME_DATA, timeout: i32) -> Result<(), CameraError> {
        let gx_get_image: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE, p_frame_data: *mut GX_FRAME_DATA, timeout: i32) -> i32> = self.lib.get(b"GXGetImage")?;
        println!("p_frame_data: {:?}", p_frame_data);
        println!("frame_data: {:?}", *p_frame_data);
        let status_code = gx_get_image(device, p_frame_data, timeout);
        let status = convert_to_gx_status(status_code);
        match status {
            GX_STATUS_LIST::GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!("Failed to get image with status: {:?}", status)))
        }
    }
    
    pub unsafe fn gx_close_device(&self, device: GX_DEV_HANDLE) -> Result<i32, libloading::Error> {
        let gx_close_device: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE) -> i32> = self.lib.get(b"GXCloseDevice")?;
        Ok(gx_close_device(device))
    }

    pub unsafe fn gx_close_lib(&self) -> Result<(), libloading::Error> {
        let gx_close_lib: Symbol<unsafe extern "C" fn() -> i32> = self.lib.get(b"GXCloseLib")?;
        gx_close_lib();
        Ok(())
    }

    // selfadded
    pub unsafe fn gx_get_int(&self, device: GX_DEV_HANDLE, feature_id: GX_FEATURE_ID, int_value: *mut i64) -> Result<i32, libloading::Error> {
        let gx_get_int: Symbol<unsafe extern "C" fn(device: GX_DEV_HANDLE, feature_id: GX_FEATURE_ID, int_value: *mut i64) -> i32> = self.lib.get(b"GXGetInt")?;
        println!("int_value: {:?}", int_value);
        Ok(gx_get_int(device, feature_id, int_value))
    }

}


// 相关定义如下
// pub type GX_DEV_HANDLE = *mut c_void;
// #[repr(C)]
// #[derive(Debug, Clone)]
// pub struct GX_FRAME_DATA {
//     pub status: u32,            // Image acquisition status
//     pub frame_id: u64,          // Frame ID
//     pub p_img_buf: *mut c_void, // Pointer to the image buffer
//     pub img_size: i32,          // Size of the image buffer, adjusted to i32 to match C definition
//     pub width: i32,             // Image width, adjusted to i32 to match C definition
//     pub height: i32,            // Image height, adjusted to i32 to match C definition
//     pub pixel_format: i32,      // Pixel format, adjusted to i32 to match C definition
//     pub timestamp: u64,         // Timestamp of the frame
//     pub offset_x: i32,          // X offset of the image
//     pub offset_y: i32,          // Y offset of the image
//     pub reserved: [i32; 1],     // Reserved, array of 1 i32 to match C definition
// }
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_STATUS_LIST {
//     GX_STATUS_SUCCESS = 0,
//     GX_STATUS_ERROR = -1,
//     GX_STATUS_NOT_FOUND_TL = -2,
//     GX_STATUS_NOT_FOUND_DEVICE = -3,
//     GX_STATUS_OFFLINE = -4,
//     GX_STATUS_INVALID_PARAMETER = -5,
//     GX_STATUS_INVALID_HANDLE = -6,
//     GX_STATUS_INVALID_CALL = -7,
//     GX_STATUS_INVALID_ACCESS = -8,
//     GX_STATUS_NEED_MORE_BUFFER = -9,
//     GX_STATUS_ERROR_TYPE = -10,
//     GX_STATUS_OUT_OF_RANGE = -11,
//     GX_STATUS_NOT_IMPLEMENTED = -12,
//     GX_STATUS_NOT_INIT_API = -13,
//     GX_STATUS_TIMEOUT = -14,
// }