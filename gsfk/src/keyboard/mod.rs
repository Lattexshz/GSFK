#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "linux")]
pub(crate) use self::linux::*;

#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;

#[cfg(target_os = "windows")]
pub(crate) mod windows;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::*;

pub type VirtualKeyCode = u32;

pub const GSFK_KEY_A: VirtualKeyCode = vk::GSFK_KEY_A as VirtualKeyCode;
pub const GSFK_KEY_B: VirtualKeyCode = vk::GSFK_KEY_B as VirtualKeyCode;
pub const GSFK_KEY_C: VirtualKeyCode = vk::GSFK_KEY_C as VirtualKeyCode;
pub const GSFK_KEY_D: VirtualKeyCode = vk::GSFK_KEY_D as VirtualKeyCode;
pub const GSFK_KEY_E: VirtualKeyCode = vk::GSFK_KEY_E as VirtualKeyCode;
pub const GSFK_KEY_F: VirtualKeyCode = vk::GSFK_KEY_F as VirtualKeyCode;
pub const GSFK_KEY_G: VirtualKeyCode = vk::GSFK_KEY_G as VirtualKeyCode;
pub const GSFK_KEY_H: VirtualKeyCode = vk::GSFK_KEY_H as VirtualKeyCode;
pub const GSFK_KEY_I: VirtualKeyCode = vk::GSFK_KEY_I as VirtualKeyCode;
pub const GSFK_KEY_J: VirtualKeyCode = vk::GSFK_KEY_J as VirtualKeyCode;
pub const GSFK_KEY_K: VirtualKeyCode = vk::GSFK_KEY_K as VirtualKeyCode;
pub const GSFK_KEY_L: VirtualKeyCode = vk::GSFK_KEY_L as VirtualKeyCode;
pub const GSFK_KEY_M: VirtualKeyCode = vk::GSFK_KEY_M as VirtualKeyCode;
pub const GSFK_KEY_N: VirtualKeyCode = vk::GSFK_KEY_N as VirtualKeyCode;
pub const GSFK_KEY_O: VirtualKeyCode = vk::GSFK_KEY_O as VirtualKeyCode;
pub const GSFK_KEY_P: VirtualKeyCode = vk::GSFK_KEY_P as VirtualKeyCode;
pub const GSFK_KEY_Q: VirtualKeyCode = vk::GSFK_KEY_Q as VirtualKeyCode;
pub const GSFK_KEY_R: VirtualKeyCode = vk::GSFK_KEY_R as VirtualKeyCode;
pub const GSFK_KEY_S: VirtualKeyCode = vk::GSFK_KEY_S as VirtualKeyCode;
pub const GSFK_KEY_T: VirtualKeyCode = vk::GSFK_KEY_T as VirtualKeyCode;
pub const GSFK_KEY_U: VirtualKeyCode = vk::GSFK_KEY_U as VirtualKeyCode;
pub const GSFK_KEY_V: VirtualKeyCode = vk::GSFK_KEY_V as VirtualKeyCode;
pub const GSFK_KEY_W: VirtualKeyCode = vk::GSFK_KEY_W as VirtualKeyCode;
pub const GSFK_KEY_X: VirtualKeyCode = vk::GSFK_KEY_X as VirtualKeyCode;
pub const GSFK_KEY_Y: VirtualKeyCode = vk::GSFK_KEY_Y as VirtualKeyCode;
pub const GSFK_KEY_Z: VirtualKeyCode = vk::GSFK_KEY_Z as VirtualKeyCode;

pub const GSFK_KEY_BACKSPACE: VirtualKeyCode = vk::GSFK_KEY_BACKSPACE as VirtualkeyCode;
pub const GSFK_KEY_SHIFT:VirtualKeyCode = vk::GSFK_KEY_SHIFT as VirtualKeyCode;
pub const GSFK_KEY_TAB: VirtualKeyCode = vk::GSFK_KEY_TAB as VirtualkeyCode;

pub fn get_key_name(code: VirtualKeyCode) {
    _get_key_name(code);
}