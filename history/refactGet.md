
我接下来想要实现gx_get_image这个方法，参照文档如下
    7.4.47. GXGetImage
声明：
GX_API GXGetImage (GX_DEV_HANDLE hDevice,
GX_FRAME_DATA *pFrameData,
int32_t nTimeout)
意义：
在开始采集之后，通过此接口可以直接获取图像，注意此接口不能与回调采集方式混用。
形参：
[in]hDevice 设备句柄
[in,out]pFrameData 指向用户传入的用来接收图像数据的地址指针
[in]nTimeout 取图的超时时间（单位 ms）
7.GxIAPI 库说明
版权所有©中国大恒（集团）有限公司北京图像视觉技术分公司 144
返回值：
GX_STATUS_SUCCESS 操作成功，没有发生错误
GX_STATUS_NOT_INIT_API 没有调用GXInitLib初始化库
GX_STATUS_INVALID_HANDLE 用户传入非法的句柄
GX_STATUS_INVALID_CALL 用户注册采集回调函数之后，又调用GXGetImage接口取图
GX_STATUS_INVALID_PARAMETER 用户传入图像地址指针为NULL

这个是他相关的结构体
7.3.5. GX_FRAME_DATA
相关接口：GXGetImage
typedef struct GX_FRAME_DATA
{
GX_FRAME_STATUS nStatus;
void* pImgBuf;
int32_t nWidth;
int32_t nHeight;
int32_t nPixelFormat;
int32_t nImgSize;
uint64_t nFrameID;
uint64_t nTimestamp;
int32_t nOffsetX;
int32_t nOffsetY;
int32_t reserved[1];
}GX_FRAME_DATA;
名称
描述
nStatus
标示获取的图像的状态，参考 GX_FRAME_STATUS
pImgBuf
图像数据指针（开启帧信息后，pImgBuf 包含图像数据和帧信息数据）
nWidth
图像宽度
nHeight
图像高度
nPixelFormat
图像格式
7.GxIAPI 库说明
版权所有©中国大恒（集团）有限公司北京图像视觉技术分公司 98
nImgSize
数据大小（开启帧信息后，nImgsize 为图像数据大小+帧信息大小）
nFrameID
图像的帧号
nTimestamp
图像的时间戳
nOffsetX
图像 X 方向偏移
nOffsetY
图像 Y 方向偏移
reserved
4 字节，保留

在我已有的代码里，
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
，我的main.rs目前是这样的！
use image::{ImageBuffer, Luma, error::ImageError, error::ParameterErrorKind, error::ParameterError};
use std::path::Path;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::mem::size_of;
use std::slice;

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


请你帮我实现我的需求：
1. 定义一个结构体GX_FRAME_DATA，用来接收图像数据
2. 实现一个方法gx_get_image，用来获取图像数据
3. 配置参数，宽为640、高度为480、格式为PixelFormatEntry::BayerRg8
4. 在main.rs里调用gx_get_image方法，获取图像数据
用中文讲解，谢谢！