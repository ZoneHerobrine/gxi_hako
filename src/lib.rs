pub mod gx {
    pub mod gx_const;
    pub mod gx_enum;
    pub mod gx_handle;
    pub mod gx_interface;
    pub mod gx_pixel_format;
    pub mod gx_struct;
}

pub use gx::{
    gx_interface::GXInterface, 
    gx_enum::*,
    gx_struct::*,
};