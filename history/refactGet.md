已知
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_DEVICE_BASE_INFO {
    pub szVendorName: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szModelName: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szSN: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szDisplayName: [u8; GX_INFO_LENGTH_128_BYTE],
    pub szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE],
    pub szUserID: [u8; GX_INFO_LENGTH_64_BYTE],
    pub accessStatus: GX_ACCESS_STATUS_CMD,
    pub deviceClass: GX_DEVICE_CLASS,
    pub reserved: [u8; 300],
}

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
            _ => Err(CameraError::OperationError(format!("GXSendCommand failed with status: {}", status_code))),
        }
    }
    