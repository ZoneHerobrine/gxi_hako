#![allow(dead_code)]


//------------------------------------------------------------------------------
//  Frame Status Definition
//------------------------------------------------------------------------------
// 这种写法的好处暂时搁置
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GXFrameStatusList {
//     GX_FRAME_STATUS_SUCCESS = 0,           // Normal frame
//     GX_FRAME_STATUS_INCOMPLETE = -1,       // Incomplete frame
// }
// pub type GX_FRAME_STATUS = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS {
    GX_FRAME_STATUS_SUCCESS = 0,           // Normal frame
    GX_FRAME_STATUS_INCOMPLETE = -1,       // Incomplete frame
}


//------------------------------------------------------------------------------
//  Device Type Definition
//------------------------------------------------------------------------------
// 历史原因保留下面的写法
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_DEVICE_CLASS {
//     GX_DEVICE_CLASS_UNKNOWN = 0,           // Unknown device type
//     GX_DEVICE_CLASS_USB2 = 1,              // USB2.0 Vision device
//     GX_DEVICE_CLASS_GEV = 2,               // Gige Vision device
//     GX_DEVICE_CLASS_U3V = 3,               // USB3 Vision device
// }
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS {
    Unknown = 0,
    Usb2 = 1,
    Gev = 2,
    U3v = 3,
}

//------------------------------------------------------------------------------
//  Feature Type Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_TYPE {
    GX_FEATURE_INT = 0x10000000,           // Integer type
    GX_FEATURE_FLOAT = 0x20000000,         // Floating point type
    GX_FEATURE_ENUM = 0x30000000,          // Enum type
    GX_FEATURE_BOOL = 0x40000000,          // Boolean type
    GX_FEATURE_STRING = 0x50000000,        // String type
    GX_FEATURE_BUFFER = 0x60000000,        // Block data type
    GX_FEATURE_COMMAND = 0x70000000,       // Command type
}

//------------------------------------------------------------------------------
//  Feature Level Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_LEVEL {
    GX_FEATURE_LEVEL_REMOTE_DEV         = 0x00000000,  //< Remote device layer
    GX_FEATURE_LEVEL_TL                 = 0x01000000,  //< TL layer
    GX_FEATURE_LEVEL_IF                 = 0x02000000,  //< Interface layer    
    GX_FEATURE_LEVEL_DEV                = 0x03000000,  //< Device layer
    GX_FEATURE_LEVEL_DS                 = 0x04000000,  //< DataStream layer
}

//------------------------------------------------------------------------------
//  Access Mode of Device
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_MODE
{
    GX_ACCESS_READONLY                  = 2,           //< Open the device in read-only mode
    GX_ACCESS_CONTROL                   = 3,           //< Open the device in controlled mode
    GX_ACCESS_EXCLUSIVE                 = 4,           //< Open the device in exclusive mode
}
// 历史原因保留
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_STATUS_CMD {
    Unknown = 0,
    ReadWrite = 1,
    ReadOnly = 2,
    NoAccess = 3,
}


// 自己写的
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_ID{
    // GX_COMMAND_ACQUISITION_START = 3001 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV,
    // GX_COMMAND_ACQUISITION_STOP = 3002| GX_FEATURE_TYPE::GX_FEATURE_COMMAND | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV,
    // GX_COMMAND_TRIGGER_SOFTWARE = 3006| GX_FEATURE_TYPE::GX_FEATURE_COMMAND | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV,
    GX_COMMAND_ACQUISITION_START = 3001 | 1879048192 | 0,
    GX_COMMAND_ACQUISITION_STOP = 3002| 1879048192 | 0,
    GX_COMMAND_TRIGGER_SOFTWARE = 3006| 1879048192 | 0,
    // GX_INT_WIDTH                      = 1006 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Width of the image provided by the device (in pixels).
    // GX_INT_HEIGHT                     = 1007 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Height of the image provided by the device (in pixels).
    GX_INT_WIDTH = 1006 | 268435456 | 0,
    GX_INT_HEIGHT = 1007 | 268435456 | 0,
    GX_INT_PAYLOAD_SIZE = 2000 | 268435456 | 0,

    GX_ENUM_PIXEL_SIZE                = 1012 | 805306368 | 0,    //< Total size in bits of a pixel of the image.
    GX_ENUM_PIXEL_FORMAT              = 1014 | 805306368 | 0,    //< Format of the pixels provided by the device.
   
    // 下面有进制转换问题，待修复
    //---------------ImageFormat Section--------------------------------
    // GX_INT_SENSOR_WIDTH               = 1000 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Effective width of the sensor in pixels.
    // GX_INT_SENSOR_HEIGHT              = 1001 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Effective height of the sensor in pixels.
    // GX_INT_WIDTH_MAX                  = 1002 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Maximum width of the image (in pixels).
    // GX_INT_HEIGHT_MAX                 = 1003 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Maximum height of the image (in pixels).
    // GX_INT_OFFSET_X                   = 1004 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Horizontal offset from the origin to the region of interest (in pixels).
    // GX_INT_OFFSET_Y                   = 1005 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Vertical offset from the origin to the region of interest (in pixels).
    // GX_INT_WIDTH                      = 1006 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Width of the image provided by the device (in pixels).
    // GX_INT_HEIGHT                     = 1007 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Height of the image provided by the device (in pixels).
    // GX_INT_BINNING_HORIZONTAL         = 1008 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Number of horizontal photo-sensitive cells to combine together.
    // GX_INT_BINNING_VERTICAL           = 1009 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Number of vertical photo-sensitive cells to combine together.
    // GX_INT_DECIMATION_HORIZONTAL      = 1010 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Horizontal sub-sampling of the image.
    // GX_INT_DECIMATION_VERTICAL        = 1011 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,     //< Vertical sub-sampling of the image.
    // GX_ENUM_PIXEL_SIZE                = 1012 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Total size in bits of a pixel of the image.
    // GX_ENUM_PIXEL_COLOR_FILTER        = 1013 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Type of color filter that is applied to the image.
    // GX_ENUM_PIXEL_FORMAT              = 1014 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Format of the pixels provided by the device.
    // GX_BOOL_REVERSE_X                 = 1015 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Flip horizontally the image sent by the device.
    // GX_BOOL_REVERSE_Y                 = 1016 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Flip vertically the image sent by the device.
    // GX_ENUM_TEST_PATTERN              = 1017 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Selects the type of test pattern that is generated by the device as image source.
    // GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR = 1018 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,   //< Selects which test pattern generator is controlled by the TestPattern feature.
    // GX_ENUM_REGION_SEND_MODE          = 1019 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< ROI output mode, see also GX_REGION_SEND_MODE_ENTRY
    // GX_ENUM_REGION_MODE               = 1020 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< zone switch, see also GX_REGION_MODE_ENTRY
    // GX_ENUM_RREGION_SELECTOR          = 1021 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Selects the Region of interest to control.
    // GX_INT_CENTER_WIDTH               = 1022 |GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,      //< width of window
    // GX_INT_CENTER_HEIGHT              = 1023 |GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,      //< height of window
    // GX_ENUM_BINNING_HORIZONTAL_MODE   = 1024 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Binning Horizontal mode, see also GX_BINNING_HORIZONTAL_MODE_ENTRY
    // GX_ENUM_BINNING_VERTICAL_MODE     = 1025 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Binning vertical mode, see also GX_BINNING_VERTICAL_MODE_ENTRY
	// GX_ENUM_SENSOR_SHUTTER_MODE		  = 1026 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,    //< Sensor shutter mode, refer to GX_SENSOR_SHUTTER_MODE_ENTRY
	// GX_INT_DECIMATION_LINENUMBER      = 1027 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,    //< decimation line number
	// GX_INT_SENSOR_DECIMATION_HORIZONTAL = 1028 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,  //< Sensor horizontal decimation
	// GX_INT_SENSOR_DECIMATION_VERTICAL   = 1029 | GX_FEATURE_INT  | GX_FEATURE_LEVEL_REMOTE_DEV,  //< Sensor vertical decimation
	// GX_ENUM_SENSOR_SELECTOR             = 1030 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,  //< selector current sonsor, refer to GX_SENSOR_SELECTOR_ENTRY
	// GX_INT_CURRENT_SENSOR_WIDTH         = 1031 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor width
	// GX_INT_CURRENT_SENSOR_HEIGHT        = 1032 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor height
	// GX_INT_CURRENT_SENSOR_OFFSETX       = 1033 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor offset X
	// GX_INT_CURRENT_SENSOR_OFFSETY       = 1034 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor offset Y
	// GX_INT_CURRENT_SENSOR_WIDTHMAX      = 1035 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor width max
	// GX_INT_CURRENT_SENSOR_HEIGHTMAX     = 1036 | GX_FEATURE_INT | GX_FEATURE_LEVEL_REMOTE_DEV,   //< current sonsor height max
	// GX_ENUM_SENSOR_BIT_DEPTH			= 1037 | GX_FEATURE_ENUM | GX_FEATURE_LEVEL_REMOTE_DEV,	 //< Sensor Bit Depth, refer to GX_SENSOR_BIT_DEPTH_ENTRY
	// GX_BOOL_WATERMARK_ENABLE			= 1038 | GX_FEATURE_BOOL | GX_FEATURE_LEVEL_REMOTE_DEV,  //< Watermark



}

// 下面是3.5写的，大多数应该是用不到的


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_STATUS_LIST {
    GX_STATUS_SUCCESS = 0,
    GX_STATUS_ERROR = -1,
    GX_STATUS_NOT_FOUND_TL = -2,
    GX_STATUS_NOT_FOUND_DEVICE = -3,
    GX_STATUS_OFFLINE = -4,
    GX_STATUS_INVALID_PARAMETER = -5,
    GX_STATUS_INVALID_HANDLE = -6,
    GX_STATUS_INVALID_CALL = -7,
    GX_STATUS_INVALID_ACCESS = -8,
    GX_STATUS_NEED_MORE_BUFFER = -9,
    GX_STATUS_ERROR_TYPE = -10,
    GX_STATUS_OUT_OF_RANGE = -11,
    GX_STATUS_NOT_IMPLEMENTED = -12,
    GX_STATUS_NOT_INIT_API = -13,
    GX_STATUS_TIMEOUT = -14,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS_LIST {
    GX_FRAME_STATUS_SUCCESS = 0,
    GX_FRAME_STATUS_INCOMPLETE = -1,
}

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FRAME_STATUS {
//     GX_FRAME_STATUS_SUCCESS = 0,
//     GX_FRAME_STATUS_INCOMPLETE = -1,
// }


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS_LIST {
    GX_DEVICE_CLASS_UNKNOWN = 0,
    GX_DEVICE_CLASS_USB2 = 1,
    GX_DEVICE_CLASS_GEV = 2,
    GX_DEVICE_CLASS_U3V = 3,
}


// #[repr(u32)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FEATURE_TYPE {
//     GX_FEATURE_INT = 0x10000000,
//     GX_FEATURE_FLOAT = 0x20000000,
//     GX_FEATURE_ENUM = 0x30000000,
//     GX_FEATURE_BOOL = 0x40000000,
//     GX_FEATURE_STRING = 0x50000000,
//     GX_FEATURE_BUFFER = 0x60000000,
//     GX_FEATURE_COMMAND = 0x70000000,
// }

// #[repr(u32)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_FEATURE_LEVEL {
//     GX_FEATURE_LEVEL_REMOTE_DEV = 0x00000000,
//     GX_FEATURE_LEVEL_TL = 0x01000000,
//     GX_FEATURE_LEVEL_IF = 0x02000000,
//     GX_FEATURE_LEVEL_DEV = 0x03000000,
//     GX_FEATURE_LEVEL_DS = 0x04000000,
// }

// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub enum GX_ACCESS_MODE {
//     GX_ACCESS_READONLY = 2,
//     GX_ACCESS_CONTROL = 3,
//     GX_ACCESS_EXCLUSIVE = 4,
// }

pub type GX_ACCESS_MODE_CMD = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_STATUS {
    GX_ACCESS_STATUS_UNKNOWN = 0,
    GX_ACCESS_STATUS_READWRITE = 1,
    GX_ACCESS_STATUS_READONLY = 2,
    GX_ACCESS_STATUS_NOACCESS = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_OPEN_MODE {
    GX_OPEN_SN = 0,
    GX_OPEN_IP = 1,
    GX_OPEN_MAC = 2,
    GX_OPEN_INDEX = 3,
    GX_OPEN_USERID = 4,
}

pub type GX_OPEN_MODE_CMD = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_IP_CONFIGURE_MODE_LIST {
    GX_IP_CONFIGURE_DHCP,
    GX_IP_CONFIGURE_LLA,
    GX_IP_CONFIGURE_STATIC_IP,
    GX_IP_CONFIGURE_DEFAULT,
}

pub type GX_IP_CONFIGURE_MODE = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_RESET_DEVICE_MODE {
    GX_MANUFACTURER_SPECIFIC_RECONNECT = 0x1,
    GX_MANUFACTURER_SPECIFIC_RESET = 0x2,
}

pub type GX_RESET_DEVICE_MODE_CMD = i32;
