use super::gx_const::*;
use super::gx_enum::*;
use std::ffi::{c_char,c_void};

// 请你放照下面的格式，把上述C开发文档的内容写为rust版本，谢谢！
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_DEVICE_IP_INFO {
    pub szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE + 4],
    pub szMAC: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szIP: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szSubNetMask: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szGateWay: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICMAC: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICIP: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICSubNetMask: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICGateWay: [u8; GX_INFO_LENGTH_32_BYTE],
    pub szNICDescription: [u8; GX_INFO_LENGTH_128_BYTE + 4],
    pub reserved: [u8; 512],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_OPEN_PARAM {
    pub pszContent: *const c_char,
    pub openMode: GX_OPEN_MODE_CMD,
    pub accessMode: GX_ACCESS_MODE_CMD,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FRAME_CALLBACK_PARAM {
    pub pUserParam: *mut c_void,
    pub status: GX_FRAME_STATUS,
    pub pImgBuf: *const c_void,
    pub nImgSize: i32,
    pub nWidth: i32,
    pub nHeight: i32,
    pub nPixelFormat: i32,
    pub nFrameID: u64,
    pub nTimestamp: u64,
    pub reserved: [i32; 1],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FRAME_DATA {
    pub status: u32,            // Image acquisition status
    pub frame_id: u64,          // Frame ID
    pub p_img_buf: *mut c_void, // Pointer to the image buffer
    pub img_size: i32,          // Size of the image buffer, adjusted to i32 to match C definition
    pub width: i32,             // Image width, adjusted to i32 to match C definition
    pub height: i32,            // Image height, adjusted to i32 to match C definition
    pub pixel_format: i32,      // Pixel format, adjusted to i32 to match C definition
    pub timestamp: u64,         // Timestamp of the frame
    pub offset_x: i32,          // X offset of the image
    pub offset_y: i32,          // Y offset of the image
    pub reserved: [i32; 1],     // Reserved, array of 1 i32 to match C definition
}
// pub struct GX_FRAME_DATA {
//     pub status: u32,            // Image acquisition status
//     pub frame_id: u64,          // Frame ID
//     pub p_img_buf: *mut c_void,     // Pointer to the image buffer
//     pub img_size: usize,        // Size of the image buffer
//     pub width: u32,             // Image width
//     pub height: u32,            // Image height
//     pub pixel_format: u32,      // Pixel format
//     pub timestamp: u64,         // Timestamp of the frame
// }
// pub struct GX_FRAME_DATA {
//     pub nStatus: GX_FRAME_STATUS,
//     pub pImgBuf: *mut c_void,
//     pub nWidth: i32,
//     pub nHeight: i32,
//     pub nPixelFormat: i32,
//     pub nImgSize: i32,
//     pub nFrameID: u64,
//     pub nTimestamp: u64,
//     pub nOffsetX: i32,
//     pub nOffsetY: i32,
//     pub reserved: [i32; 1],
// }

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FRAME_BUFFER {
    pub nStatus: GX_FRAME_STATUS,
    pub pImgBuf: *mut c_void,
    pub nWidth: i32,
    pub nHeight: i32,
    pub nPixelFormat: i32,
    pub nImgSize: i32,
    pub nFrameID: u64,
    pub nTimestamp: u64,
    pub nBufID: u64,
    pub nOffsetX: i32,
    pub nOffsetY: i32,
    pub reserved: [i32; 16],
}

pub type PGX_FRAME_BUFFER = *mut GX_FRAME_BUFFER;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_INT_RANGE {
    pub nMin: i64,
    pub nMax: i64,
    pub nInc: i64,
    pub reserved: [i32; 8],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_FLOAT_RANGE {
    pub dMin: f64,
    pub dMax: f64,
    pub dInc: f64,
    pub szUnit: [c_char; GX_INFO_LENGTH_8_BYTE],
    pub bIncIsValid: bool,
    pub reserved: [i8; 31],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_ENUM_DESCRIPTION {
    pub nValue: i64,
    pub szSymbolic: [c_char; GX_INFO_LENGTH_64_BYTE],
    pub reserved: [i32; 8],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GX_REGISTER_STACK_ENTRY {
    pub nAddress: u64,
    pub pBuffer: *mut c_void,
    pub nSize: usize,
}
