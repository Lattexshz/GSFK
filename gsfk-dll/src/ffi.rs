#[macro_export]
macro_rules! value_from_ptr {
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