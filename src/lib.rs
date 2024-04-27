//! This library provides a rust interface to the GX API (Daheng Camera).
//! 
//! # Quick Start
//! 
//! ```rust
//! use std::mem::size_of;
//! use gxi_hako::{
//!    gx::{
//!         gx_interface::*,
//!         gx_enum::*,
//!         gx_struct::*,
//!         gx_handle::*,
//!     },
//!     utils::{
//!         debug::print_device_info,
//!         builder::GXDeviceBaseInfoBuilder,
//!     }
//! };
//! 
//! fn main() {
//!     unsafe {
//!         // You can change the library path as you need
//!         let gx = GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll").expect("Failed to load library");
//!     
//!         gx.gx_init_lib().expect("Failed to initialize library");
//!     
//!         let mut device_num = 0;
//!         gx.gx_update_device_list(&mut device_num, 1000)
//!             .expect("Failed to update device list");
//!     
//!         if device_num > 0 {
//!             let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num).map(|_| {
//!                 GXDeviceBaseInfoBuilder::new().build()
//!             }).collect();
//!             let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
//!             let status = gx
//!                 .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)
//!                 .expect("Failed to get all device base info");
//!             if status == 0 {
//!                 // Assuming 0 is GX_STATUS_SUCCESS
//!                 println!(
//!                     "Device base info retrieved successfully. Number of devices: {}",
//!                     device_num
//!                 );
//!                 for device in &base_info {
//!                     print_device_info(&device);
//!                 }
//!             } else {
//!                 println!("Failed to retrieve device base info, status: {}", status);
//!             }
//!         } else {
//!             println!("No Devices found.");
//!         }
//!         gx.gx_close_lib().expect("Failed to close library");
//!         println!("Library closed.")
//!     }
//! }
//! ```
//! 
//! # Feature Overview
//! 
//! ## GX
//! 
//! The gx module contains the following modules:
//! - gx_const: Contains the constants used in the GX API.
//! - gx_callback: Contains the callback function for the frame data.
//! - gx_enum: Contains the enums used in the GX API.
//! - gx_handle: Contains the handle for the GX API.
//! - gx_interface: Contains the interface for the GX API. All functions are defined here in the GXInterface trait.
//! - gx_pixel_format: Contains the pixel format for the GX API.
//! - gx_struct: Contains the structs used in the GX API.
//! 
//! 
//! ## Utils
//! 
//! The utils module contains the following modules:
//! - builder: Contains the builder pattern for creating the GXDeviceBaseInfo and GXOpenParam structs.
//! - cv_gui: Contains the functions for displaying images using OpenCV.
//! - debug: Contains the function for printing the device information.
//! - facade: Contains the GXFrameDataFacade struct which is used to manage the frame data.
//! - image_process: Contains the functions for processing images.
//! - status: Contains the status codes converter for the GX API.
//! 
//! 

pub mod gx {
    pub mod gx_const;
    pub mod gx_callback;
    pub mod gx_enum;
    pub mod gx_handle;
    pub mod gx_interface;
    pub mod gx_pixel_format;
    pub mod gx_struct;
}

pub mod utils {
    pub mod builder;
    pub mod cv_gui;
    pub mod image_process;
    pub mod debug;
    pub mod facade;
    pub mod status;
}

pub use gx::{
    gx_interface::{GXInstance,GXInterface}, 
    gx_enum::*,
    gx_struct::*,

};

pub use utils::{
    builder::{GXDeviceBaseInfoBuilder,GXOpenParamBuilder},
    debug::print_device_info,
    facade::GXFrameDataFacade,
    
};