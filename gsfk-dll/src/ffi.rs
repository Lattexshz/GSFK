#[macro_export]
macro_rules! ref_from_ptr {
    ($ptr:expr) => {
        if $ptr.is_null() {
            panic!("Passed pointer is null");
        } else {
            unsafe {
                &*$ptr
            }
        }
    }
}

#[macro_export]
macro_rules! BOOL {
    ($integer:expr) => {
        $integer != 0
    }
}