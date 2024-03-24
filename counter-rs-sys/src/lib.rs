#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use std::slice;

    #[test]
    fn round_trip() {
        unsafe {
            let handle = CreateInstance(42);

            // increment
            {
                let result = Increment(handle, 5);
                assert_eq!(result, 0);
                let current = GetCurrentValue(handle);
                assert_eq!(current, 47);
            }

            // decrement
            {
                let result = Decrement(handle, 46);
                assert_eq!(result, 0);
                let current = GetCurrentValue(handle);
                assert_eq!(current, 1);
            }

            // increment error
            {
                let result = Increment(handle, u32::MAX);
                assert_eq!(result, -1);
                let current = GetCurrentValue(handle);
                assert_eq!(current, 1);
            }

            // decrement error
            {
                let result = Decrement(handle, 2);
                assert_eq!(result, -1);
                let current = GetCurrentValue(handle);
                assert_eq!(current, 1);
            }

            // history
            {
                let mut length = 0usize;
                let ptr = GetHistory(handle, &mut length);
                let history = slice::from_raw_parts(ptr, length);
                assert_eq!(history, &[42, 47, 1]);

                libc::free(ptr as *mut c_void);
            }

            ReleaseInstance(handle);
        }
    }
}
