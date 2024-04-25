```rust
                    println!("Attempting to capture image...");
                    println!("Device Handle: {:?}", device_handle);
                    println!("Frame Data Pointer: {:?}", frame_data);

                    let mut width_value: i64 = 1;
                    let get_int_result =
                        gx.gx_get_int(device_handle, GX_FEATURE_ID::GX_INT_WIDTH, &mut width_value);
                    match get_int_result {
                        Ok(_) => println!("Width: {}", width_value),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }
                    let mut height_value: i64 = 1;
                    let get_int_result = gx.gx_get_int(
                        device_handle,
                        GX_FEATURE_ID::GX_INT_HEIGHT,
                        &mut height_value,
                    );
                    match get_int_result {
                        Ok(_) => println!("Height: {}", height_value),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }

                    let mut payload_size: i64 = 1;
                    let get_int_result = gx.gx_get_int(
                        device_handle,
                        GX_FEATURE_ID::GX_INT_PAYLOAD_SIZE,
                        &mut payload_size,
                    );
                    match get_int_result {
                        Ok(_) => println!("Payload Size: {}", payload_size),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }

                    let mut pixel_format: i64 = 1;
                    let get_enum_result = gx.gx_get_enum(
                        device_handle,
                        GX_FEATURE_ID::GX_ENUM_PIXEL_FORMAT,
                        &mut pixel_format,
                    );
                    match get_enum_result {
                        Ok(_) => println!("Pixel Format: {}", pixel_format),
                        Err(e) => eprintln!("Failed to get width: {:?}", e),
                    }
```