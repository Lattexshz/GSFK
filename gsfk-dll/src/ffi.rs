#[macro_export]
macro_rules! ref_from_ptr {
    ($ptr:expr) => {
        if $ptr.is_null() {
            panic!("Passed pointer is null");
        } else {
            unsafe { &*$ptr }
        }
    };
}

#[macro_export]
macro_rules! cstr_to_str {
    ($ptr:expr) => {
        match unsafe { CStr::from_ptr($ptr).to_str() } {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        }
    };
}

#[macro_export]
macro_rules! BOOL {
    ($integer:expr) => {
        $integer != 0
    };
}
