use std::ffi::{c_ulong, c_void};
use safex::glx::{glx_choose_visual, GLX_DEPTH_SIZE, GLX_DOUBLEBUFFER, glx_make_current, GLX_NONE, GLX_RGBA, GLXContext};
use safex::xlib::{AsRaw, Display, Screen};
use crate::api::gl::{OpenGLAPIDescription, OpenGLAPIExt};

pub struct _OpenGL {
    display: Display,
    screen: Screen,
    glc: GLXContext,
    window: c_ulong
}

impl _OpenGL {
    pub fn new(window: c_ulong,desc: OpenGLAPIDescription) -> Self {
        let display = Display::open(None);
        let screen = Screen::default(&display);
        let vi = glx_choose_visual(
            &display,
            &mut [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, GLX_NONE],
        ).unwrap();

        let glc = GLXContext::create(&display, &vi, None, gl::TRUE as i32);

        Self {
            display,
            screen,
            glc,
            window
        }
    }
}

impl OpenGLAPIExt for _OpenGL {
    fn make_current(&self) {
        unsafe {
            x11::glx::glXMakeCurrent(self.display.as_raw(),self.window,self.glc.as_raw());
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            println!("Swap");
            x11::glx::glXMakeCurrent(self.display.as_raw(),self.window,self.glc.as_raw());
        }
    }

    fn swap_interval(&self, interval: bool) {

    }

    fn get_proc_address(&self,addr: &str) -> *const c_void {
        self.glc.get_proc_address(addr).unwrap() as *const c_void
    }
}