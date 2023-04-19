
use raw_window_handle::RawWindowHandle;
use std::ffi::c_void;

mod sys;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

pub trait OpenGLAPIExt {
    fn make_current(&self);
    fn swap_buffers(&self);
    fn swap_interval(&self, interval: bool);
    fn get_proc_address(&self, addr: &str) -> *const c_void;
}

pub struct OpenGLAPIDescription {
    pub version_major: u32,
    pub version_minor: u32,
}

pub struct OpenGL {
    inner: _OpenGL,
}

impl OpenGL {
    pub(crate) fn new(handle: RawWindowHandle, desc: OpenGLAPIDescription) -> Self {
        let inner = match handle {
            RawWindowHandle::UiKit(_) => todo!(),
            RawWindowHandle::AppKit(_) => todo!(),
            RawWindowHandle::Orbital(_) => todo!(),
            #[cfg(target_os = "linux")]
            RawWindowHandle::Xlib(handle) => _OpenGL::new(handle.window, desc),
            RawWindowHandle::Xcb(_) => todo!(),
            RawWindowHandle::Wayland(_) => todo!(),
            RawWindowHandle::Drm(_) => todo!(),
            RawWindowHandle::Gbm(_) => todo!(),
            #[cfg(target_os = "windows")]
            RawWindowHandle::Win32(handle) => _OpenGL::new(handle.hwnd, desc),
            RawWindowHandle::WinRt(_) => todo!(),
            RawWindowHandle::Web(_) => todo!(),
            RawWindowHandle::AndroidNdk(_) => todo!(),
            RawWindowHandle::Haiku(_) => todo!(),
            _ => todo!(),
        };

        Self { inner }
    }
}

impl OpenGLAPIExt for OpenGL {
    fn make_current(&self) {
        self.inner.make_current();
    }

    fn swap_buffers(&self) {
        self.inner.swap_buffers();
    }

    fn swap_interval(&self, interval: bool) {
        self.inner.swap_interval(interval);
    }

    fn get_proc_address(&self, addr: &str) -> *const c_void {
        self.inner.get_proc_address(addr)
    }
}
