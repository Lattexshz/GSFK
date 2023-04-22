use winey::keyboard::*;

pub type VirtualKeyCode = winey::keyboard::VirtualKeyCode;

pub const GSFK_KEY_A: VirtualKeyCode = KEY_A;
pub const GSFK_KEY_B: VirtualKeyCode = KEY_B;
pub const GSFK_KEY_C: VirtualKeyCode = KEY_C;
pub const GSFK_KEY_D: VirtualKeyCode = KEY_D;
pub const GSFK_KEY_E: VirtualKeyCode = KEY_E;
pub const GSFK_KEY_F: VirtualKeyCode = KEY_F;
pub const GSFK_KEY_G: VirtualKeyCode = KEY_G;
pub const GSFK_KEY_H: VirtualKeyCode = KEY_H;
pub const GSFK_KEY_I: VirtualKeyCode = KEY_I;
pub const GSFK_KEY_J: VirtualKeyCode = KEY_J;
pub const GSFK_KEY_K: VirtualKeyCode = KEY_K;
pub const GSFK_KEY_L: VirtualKeyCode = KEY_L;
pub const GSFK_KEY_M: VirtualKeyCode = KEY_M;
pub const GSFK_KEY_N: VirtualKeyCode = KEY_N;
pub const GSFK_KEY_O: VirtualKeyCode = KEY_O;
pub const GSFK_KEY_P: VirtualKeyCode = KEY_P;
pub const GSFK_KEY_Q: VirtualKeyCode = KEY_Q;
pub const GSFK_KEY_R: VirtualKeyCode = KEY_R;
pub const GSFK_KEY_S: VirtualKeyCode = KEY_S;
pub const GSFK_KEY_T: VirtualKeyCode = KEY_T;
pub const GSFK_KEY_U: VirtualKeyCode = KEY_U;
pub const GSFK_KEY_V: VirtualKeyCode = KEY_V;
pub const GSFK_KEY_W: VirtualKeyCode = KEY_W;
pub const GSFK_KEY_X: VirtualKeyCode = KEY_X;
pub const GSFK_KEY_Y: VirtualKeyCode = KEY_Y;
pub const GSFK_KEY_Z: VirtualKeyCode = KEY_Z;

pub const GSFK_KEY_BACKSPACE: VirtualKeyCode = KEY_BACKSPACE;
pub const GSFK_KEY_SHIFT:VirtualKeyCode = KEY_SHIFT;
pub const GSFK_KEY_TAB: VirtualKeyCode = KEY_TAB;

pub fn get_key_name(code: VirtualKeyCode) -> String {
    winey::keyboard::get_key_name(code)
}