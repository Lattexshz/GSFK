use crate::api::gl::sys::{wgl, wgl_extra, WGLARBFunctions};
use crate::api::gl::{OpenGLAPIDescription, OpenGLAPIExt};


use std::ffi::{c_void, CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{addr_of, null_mut};

use winapi::shared::windef::{HGLRC, HWND};
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryW};
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

pub struct _OpenGL {
    context: HGLRC,
    func: WGLARBFunctions,
    hwnd: HWND,
}

impl _OpenGL {
    pub(crate) fn new(handle: *mut c_void, desc: OpenGLAPIDescription) -> Self {
        let hwnd = handle as HWND;
        unsafe {
            let pfd = PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: PFD_MAIN_PLANE,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let hdc = GetDC(hwnd);
            let pixel_format = ChoosePixelFormat(hdc, addr_of!(pfd));

            SetPixelFormat(hdc, pixel_format, addr_of!(pfd));

            let old_ctx = wgl::CreateContext(hdc as wgl::types::HDC);
            wgl::MakeCurrent(hdc as wgl::types::HDC, old_ctx);

            let att = [
                wgl_extra::CONTEXT_MAJOR_VERSION_ARB,
                desc.version_major,
                wgl_extra::CONTEXT_MINOR_VERSION_ARB,
                desc.version_minor,
                wgl_extra::CONTEXT_FLAGS_ARB,
                0,
                wgl_extra::CONTEXT_PROFILE_MASK_ARB,
                wgl_extra::CONTEXT_CORE_PROFILE_BIT_ARB,
                0,
            ];

            let func = WGLARBFunctions::load();
            let ctx =
                (func.wglCreateContextAttribsARB)(hdc as wgl_extra::types::HDC, null_mut(), &att);

            wgl::DeleteContext(old_ctx);

            Self {
                context: ctx as HGLRC,
                func,
                hwnd,
            }
        }
    }
}

impl OpenGLAPIExt for _OpenGL {
    fn make_current(&self) {
        unsafe {
            wgl::MakeCurrent(
                GetDC(self.hwnd) as wgl::types::HDC,
                self.context as *const c_void,
            );
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            SwapBuffers(GetDC(self.hwnd));
        }
    }

    fn swap_interval(&self, interval: bool) {
        (self.func.wglSwapIntervalEXT)(interval as u32)
    }

    fn get_proc_address(&self, addr: &str) -> *const c_void {
        let addr = CString::new(addr.as_bytes()).unwrap();
        let addr = addr.as_ptr();

        let name = OsStr::new("opengl32.dll")
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let lib = unsafe { LoadLibraryW(name.as_ptr()) };

        unsafe {
            let p = wgl::GetProcAddress(addr) as *const core::ffi::c_void;
            if !p.is_null() {
                return p;
            }
            GetProcAddress(lib, addr) as *const _
        }
    }
}
