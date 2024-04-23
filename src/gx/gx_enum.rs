// 请你放照下面的格式，把上述C开发文档的内容写为rust版本，谢谢！

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS {
    Unknown = 0,
    Usb2 = 1,
    Gev = 2,
    U3v = 3,
}

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
    GX_COMMAND_ACQUISITION_START=3001,
    GX_COMMAND_ACQUISITION_STOP=3002,
    GX_COMMAND_TRIGGER_SOFTWARE=3006,
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS {
    GX_FRAME_STATUS_SUCCESS = 0,
    GX_FRAME_STATUS_INCOMPLETE = -1,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS_LIST {
    GX_DEVICE_CLASS_UNKNOWN = 0,
    GX_DEVICE_CLASS_USB2 = 1,
    GX_DEVICE_CLASS_GEV = 2,
    GX_DEVICE_CLASS_U3V = 3,
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_TYPE {
    GX_FEATURE_INT = 0x10000000,
    GX_FEATURE_FLOAT = 0x20000000,
    GX_FEATURE_ENUM = 0x30000000,
    GX_FEATURE_BOOL = 0x40000000,
    GX_FEATURE_STRING = 0x50000000,
    GX_FEATURE_BUFFER = 0x60000000,
    GX_FEATURE_COMMAND = 0x70000000,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_LEVEL {
    GX_FEATURE_LEVEL_REMOTE_DEV = 0x00000000,
    GX_FEATURE_LEVEL_TL = 0x01000000,
    GX_FEATURE_LEVEL_IF = 0x02000000,
    GX_FEATURE_LEVEL_DEV = 0x03000000,
    GX_FEATURE_LEVEL_DS = 0x04000000,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_MODE {
    GX_ACCESS_READONLY = 2,
    GX_ACCESS_CONTROL = 3,
    GX_ACCESS_EXCLUSIVE = 4,
}

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
