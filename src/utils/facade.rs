use crate::gx::{
    gx_interface::*,
    gx_handle::GX_DEV_HANDLE,
    gx_enum::*,
    gx_struct::*,
};

use std::ffi::{CString,c_char,c_int,c_void};


pub struct GXFrameDataFacade {
    pub nStatus: c_int,
    pub pImgBuf: *mut c_void,   // Pointer to the image buffer
    pub nWidth: i32,             // Image width, adjusted to i32 to match C definition
    pub nHeight: i32,            // Image height, adjusted to i32 to match C definition
    pub nPixelFormat: i32,      // Pixel format, adjusted to i32 to match C definition
    pub nImgSize: i32,          // Size of the image buffer, adjusted to i32 to match C definition
    pub nFrameID: u64,          // Frame ID
    pub nTimestamp: u64,         // Timestamp of the frame
    pub reserved: [i32; 1],     // Reserved, array of 1 i32 to match C definition
}

// 函数用于转换 GXFrameDataFacade 到 GX_FRAME_DATA
pub fn convert_to_frame_data(facade: &GXFrameDataFacade) -> GX_FRAME_DATA {
    GX_FRAME_DATA {
        nStatus: facade.nStatus,
        pImgBuf: facade.pImgBuf,
        nWidth: facade.nWidth,
        nHeight: facade.nHeight,
        nPixelFormat: facade.nPixelFormat,
        nImgSize: facade.nImgSize,
        nFrameID: facade.nFrameID,
        nTimestamp: facade.nTimestamp,
        reserved: facade.reserved,
    }
}

pub unsafe fn fetch_frame_data(gx: &GXInstance, device_handle: GX_DEV_HANDLE) ->Result<(GXFrameDataFacade,Vec<u8>),libloading::Error> {

    let mut width_value: i64 = 0;
    let mut height_value: i64 = 0;
    let mut pixel_format: i64 = 1;
    let mut payload_size: i64 = 1;
    let mut pixel_size: i64 = 1;
    
    gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_WIDTH, &mut width_value)?;
    gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_HEIGHT, &mut height_value)?;
    gx.gx_get_enum(device_handle, GX_FEATURE_ID::GX_ENUM_PIXEL_SIZE, &mut pixel_size)?;
    gx.gx_get_enum(device_handle, GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT, &mut pixel_format)?;
    gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE, &mut payload_size)?;

    let image_size = (width_value * height_value * pixel_size) as usize;
    let mut image_buffer = vec![1u8; image_size]; // 分配图像缓冲区
    let p_img_buf = image_buffer.as_mut_ptr();

    Ok((GXFrameDataFacade {
        nStatus: 0,
        pImgBuf: p_img_buf as *mut c_void,
        nWidth: width_value as i32,
        nHeight: height_value as i32,
        nPixelFormat: pixel_format as i32,
        nImgSize: image_size as i32,
        nFrameID: 0,
        nTimestamp: 0,
        reserved: [pixel_format as i32],
    },
    image_buffer))
}

pub struct DataBundle {
    pub int_value: i64,
    pub float_value: f64,
    pub enum_value: i64,
    pub bool_value: bool,
    pub string_value: String,
}

pub fn fetch_all_data_test(instance: &GXInstance, device: GX_DEV_HANDLE) -> Result<DataBundle, libloading::Error> {
    let feature_id: GX_FEATURE_ID = GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START; // Example feature ID
    
    // Unsafe block required as we are dealing with raw pointers and external functions
    unsafe {
        // Fetch integer value
        let mut int_value: i64 = 0;
        instance.gx_get_int(device, feature_id, &mut int_value)?;

        // Fetch float value
        let mut float_value: f64 = 0.0;
        instance.gx_get_float(device, feature_id, &mut float_value)?;

        // Fetch enum value
        let mut enum_value: i64 = 0;
        instance.gx_get_enum(device, feature_id, &mut enum_value)?;

        // Fetch boolean value
        let mut bool_value: bool = false;
        instance.gx_get_bool(device, feature_id, &mut bool_value)?;

        // Fetch string value
        let mut string_length: usize = 0;
        instance.gx_get_string_length(device, feature_id, &mut string_length)?;
        let mut content = vec![0 as c_char; string_length];
        instance.gx_get_string(device, feature_id, content.as_mut_ptr(), &mut string_length)?;
        let string_value = CString::from_raw(content.as_mut_ptr()).into_string().unwrap_or_default();

        Ok(DataBundle {
            int_value,
            float_value,
            enum_value,
            bool_value,
            string_value,
        })
    }
}
