#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_close() {
        unsafe {
            assert_eq!(HI_TDE2_Open(), HI_SUCCESS as HI_S32);
            HI_TDE2_Close();
        }
    }
}
