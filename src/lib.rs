#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    use std::ptr;

    #[test]
    fn connected_devices() {
        unsafe {
            let mut err : *mut rs2_error = ptr::null_mut();
            
            let api: i32 = RS2_API_VERSION as i32;
            let ctx = rs2_create_context(api, &mut err as *mut *mut rs2_error);
            let device_list = rs2_query_devices( ctx, &mut err as *mut *mut rs2_error );
            //let errMsg = rs2_get_error_message( err );
            let num = rs2_get_device_count(device_list, &mut err as *mut *mut rs2_error);
            rs2_delete_device_list( device_list );
            rs2_delete_context( ctx );
            rs2_free_error( err );

            assert!(num >= 0);
        }
    }
}
