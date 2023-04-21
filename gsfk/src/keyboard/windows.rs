use std::ffi::{c_char, c_int, CStr};
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{GetKeyNameTextA, GetKeyNameTextW, MapVirtualKeyA, MapVirtualKeyW, MAPVK_VK_TO_VSC, VK_SHIFT, VK_TAB};
use crate::keyboard::VirtualKeyCode;

pub mod vk {
    use std::ffi::c_int;
    use winapi::um::winuser::*;

    pub const GSFK_KEY_A: c_int = 0x41;
    pub const GSFK_KEY_B: c_int = 0x42;
    pub const GSFK_KEY_C: c_int = 0x43;
    pub const GSFK_KEY_D: c_int = 0x44;
    pub const GSFK_KEY_E: c_int = 0x45;
    pub const GSFK_KEY_F: c_int = 0x46;
    pub const GSFK_KEY_G: c_int = 0x47;
    pub const GSFK_KEY_H: c_int = 0x48;
    pub const GSFK_KEY_I: c_int = 0x49;
    pub const GSFK_KEY_J: c_int = 0x4A;
    pub const GSFK_KEY_K: c_int = 0x4B;
    pub const GSFK_KEY_L: c_int = 0x4C;
    pub const GSFK_KEY_M: c_int = 0x4D;
    pub const GSFK_KEY_N: c_int = 0x4E;
    pub const GSFK_KEY_O: c_int = 0x4F;
    pub const GSFK_KEY_P: c_int = 0x50;
    pub const GSFK_KEY_Q: c_int = 0x51;
    pub const GSFK_KEY_R: c_int = 0x52;
    pub const GSFK_KEY_S: c_int = 0x53;
    pub const GSFK_KEY_T: c_int = 0x54;
    pub const GSFK_KEY_U: c_int = 0x55;
    pub const GSFK_KEY_V: c_int = 0x56;
    pub const GSFK_KEY_W: c_int = 0x57;
    pub const GSFK_KEY_X: c_int = 0x58;
    pub const GSFK_KEY_Y: c_int = 0x59;
    pub const GSFK_KEY_Z: c_int = 0x5A;

    pub const GSFK_KEY_BACKSPACE: c_int = VK_BACK;
    pub const GSFK_KEY_TAB: c_int = VK_TAB;
    pub const GSFK_KEY_SHIFT: c_int = VK_SHIFT;
}

pub fn _get_key_name(code: VirtualKeyCode) -> String {
    unsafe {
        let mut name: [i8;32] = Default::default();
        let mut ptr = name.as_mut_ptr();
        let code = MapVirtualKeyA(code, MAPVK_VK_TO_VSC);
        GetKeyNameTextA((code << 16) as i32,ptr,32);
        let string = CStr::from_ptr(ptr as *const c_char).to_str().unwrap().to_owned();
        string
    }
}