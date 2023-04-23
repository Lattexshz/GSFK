use crate::api::gl::{GLProfile, OpenGLAPIDescription, OpenGLAPIExt};
use safex::glx::{
    glx_choose_visual, glx_make_current, GLXContext, GLX_DEPTH_SIZE, GLX_DOUBLEBUFFER, GLX_NONE,
    GLX_RGBA,
};
use safex::xlib::{AsRaw, Display, Screen};
use std::ffi::{c_int, c_ulong, c_void};
use x11::glx::arb::{GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB, GLX_CONTEXT_CORE_PROFILE_BIT_ARB, GLX_CONTEXT_MAJOR_VERSION_ARB, GLX_CONTEXT_MINOR_VERSION_ARB, GLX_CONTEXT_PROFILE_MASK_ARB};

pub struct _OpenGL {
    display: Display,
    screen: Screen,
    glc: GLXContext,
    window: c_ulong,
}

impl _OpenGL {
    pub fn new(window: c_ulong, desc: OpenGLAPIDescription) -> Self {
        let display = Display::open(None);
        let screen = Screen::default(&display);
        let profile = match desc.profile {
            GLProfile::Core => GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
            GLProfile::Compatibility => GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
            GLProfile::ES => 0
        };
        let vi = glx_choose_visual(
            &display,
            &mut [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_CONTEXT_PROFILE_MASK_ARB,profile,GLX_CONTEXT_MAJOR_VERSION_ARB,desc.version_major as c_int,GLX_CONTEXT_MINOR_VERSION_ARB,desc.version_minor as c_int,GLX_NONE],
        )
        .unwrap();

        let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);

        Self {
            display,
            screen,
            glc,
            window,
        }
    }
}

impl OpenGLAPIExt for _OpenGL {
    fn make_current(&self) {
        unsafe {
            x11::glx::glXMakeCurrent(self.display.as_raw(), self.window, self.glc.as_raw());
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            x11::glx::glXSwapBuffers(self.display.as_raw(), self.window);
        }
    }

    fn swap_interval(&self, interval: bool) {}

    fn get_proc_address(&self, addr: &str) -> *const c_void {
        self.glc.get_proc_address(addr).unwrap() as *const c_void
    }
}
