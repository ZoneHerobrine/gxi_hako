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
        Ok(gx_get_all_device_base_info(p_device_info, p_buffer_size))
    }

    pub unsafe fn gx_open_device_by_index(&self, index: u32, device: *mut GX_DEV_HANDLE) -> Result<i32, libloading::Error> {
        let gx_open_device_by_index: Symbol<unsafe extern "C" fn(index: u32, device: *mut GX_DEV_HANDLE) -> i32> = self.lib.get(b"GXOpenDeviceByIndex")?;
        Ok(gx_open_device_by_index(index, device))
    }

    // Function to send a command to the camera
    pub unsafe fn gx_send_command(
        &self, 
        device: GX_DEV_HANDLE, 
        feature_id: GX_FEATURE_ID
    ) -> Result<(), CameraError> {
        let gx_send_command: Symbol<unsafe extern "C" fn(GX_DEV_HANDLE, GX_FEATURE_ID) -> i32> = self.lib.get(b"GXSendCommand")?;
        
        let status = gx_send_command(device, feature_id);
        match status {
            GX_STATUS_SUCCESS => Ok(()),
            _ => Err(CameraError::OperationError(format!("GXSendCommand failed with status: {}", status))),
        }
    }
    
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
}