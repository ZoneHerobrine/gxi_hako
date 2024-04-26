//! This library provides a rust interface to the GX API (Daheng Camera).
//! 
//! # Feature Overview
//! 

pub mod gx {
    pub mod gx_const;
    pub mod gx_enum;
    pub mod gx_handle;
    pub mod gx_interface;
    pub mod gx_pixel_format;
    pub mod gx_struct;
}

pub mod utils {
    pub mod builder;
    pub mod cv_gui;
    pub mod debug;
    pub mod facade;
    pub mod status;
}

pub use gx::{
    gx_interface::GXInstance, 
    gx_enum::*,
    gx_struct::*,

};

pub use utils::{
    builder::GXDeviceBaseInfoBuilder,
    debug::print_device_info,
    status::convert_to_gx_status,
    
};