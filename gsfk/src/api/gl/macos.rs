use std::ffi::c_void;
use crate::api::gl::OpenGLAPIExt;

pub struct _OpenGL {

}

impl OpenGLAPIExt for _OpenGL {
    fn make_current(&self) {

    }

    fn swap_buffers(&self) {

    }

    fn swap_interval(&self, interval: bool) {

    }

    fn get_proc_address(&self,addr: &str) -> *const c_void {
        std::ptr::null()
    }
}