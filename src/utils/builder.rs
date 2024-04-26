use crate::gx::
{
    gx_enum::*,
    gx_const::*,
    gx_struct::*,
};

// Builder 结构体
pub struct GXDeviceBaseInfoBuilder {
    szVendorName: [u8; GX_INFO_LENGTH_32_BYTE],
    szModelName: [u8; GX_INFO_LENGTH_32_BYTE],
    szSN: [u8; GX_INFO_LENGTH_32_BYTE],
    szDisplayName: [u8; GX_INFO_LENGTH_128_BYTE],
    szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE],
    szUserID: [u8; GX_INFO_LENGTH_64_BYTE],
    accessStatus: GX_ACCESS_STATUS_CMD,
    deviceClass: GX_DEVICE_CLASS,
    reserved: [u8; 300],
}

impl GXDeviceBaseInfoBuilder {
    pub fn new() -> Self {
        Self {
            szVendorName: [0; GX_INFO_LENGTH_32_BYTE],
            szModelName: [0; GX_INFO_LENGTH_32_BYTE],
            szSN: [0; GX_INFO_LENGTH_32_BYTE],
            szDisplayName: [0; GX_INFO_LENGTH_128_BYTE],
            szDeviceID: [0; GX_INFO_LENGTH_64_BYTE],
            szUserID: [0; GX_INFO_LENGTH_64_BYTE],
            accessStatus: GX_ACCESS_STATUS_CMD::Unknown,
            deviceClass: GX_DEVICE_CLASS::Unknown,
            reserved: [0; 300],
        }
    }

    pub fn sz_vendor_name(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szVendorName = value;
        self
    }

    pub fn sz_model_name(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szModelName = value;
        self
    }

    pub fn sz_sn(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szSN = value;
        self
    }

    pub fn sz_display_name(mut self, value: [u8; GX_INFO_LENGTH_128_BYTE]) -> Self {
        self.szDisplayName = value;
        self
    }

    pub fn sz_device_id(mut self, value: [u8; GX_INFO_LENGTH_64_BYTE]) -> Self {
        self.szDeviceID = value;
        self
    }

    pub fn sz_user_id(mut self, value: [u8; GX_INFO_LENGTH_64_BYTE]) -> Self {
        self.szUserID = value;
        self
    }

    pub fn access_status(mut self, value: GX_ACCESS_STATUS_CMD) -> Self {
        self.accessStatus = value;
        self
    }

    pub fn device_class(mut self, value: GX_DEVICE_CLASS) -> Self {
        self.deviceClass = value;
        self
    }

    pub fn reserved(mut self, value: [u8; 300]) -> Self {
        self.reserved = value;
        self
    }

    // 构建最终的 GX_DEVICE_BASE_INFO 实例
    pub fn build(self) -> GX_DEVICE_BASE_INFO {
        GX_DEVICE_BASE_INFO {
            szVendorName: self.szVendorName,
            szModelName: self.szModelName,
            szSN: self.szSN,
            szDisplayName: self.szDisplayName,
            szDeviceID: self.szDeviceID,
            szUserID: self.szUserID,
            accessStatus: self.accessStatus,
            deviceClass: self.deviceClass,
            reserved: self.reserved,
        }
    }
}
