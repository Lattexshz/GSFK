use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use winey::WindowEvent;
use winey::window::ControlFlow;

use crate::{API, APIDescription, WindowImplementation};
use crate::api::gl::{OpenGL, OpenGLAPIDescription};
use crate::api::vulkan::{Vulkan, VulkanAPIDescription};

#[repr(C)]
pub struct Window {
    inner: winey::window::Window,
}

impl Window {
    pub fn new_with_vulkan(title: &str,width: u32,height: u32) -> (Self,API<Vulkan>) {
        let inner = winey::window::Window::new(title,width,height);

        let api = API {
            context: Vulkan::new(),
        };

        (Self { inner },api)
    }

    pub fn new_with_opengl(title: &str,width: u32,height: u32,desc: OpenGLAPIDescription) -> (Self,API<OpenGL>) {
        let inner = winey::window::Window::new(title,width,height);

        let api = API {
            context: OpenGL::new(inner.raw_window_handle(),desc),
        };

        (Self { inner },api)
    }

    pub fn run<C: FnMut(WindowEvent,&mut ControlFlow)>(&self, callback: C) {
        self.inner.run(callback);
    }
}

unsafe impl HasRawWindowHandle for Window {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.inner.raw_window_handle()
    }
}

unsafe impl HasRawDisplayHandle for Window {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        self.inner.raw_display_handle()
    }
}

impl WindowImplementation for Window {
    fn show(&self) {
        self.inner.show();
    }
    fn hide(&self) {
        self.inner.hide();
    }
    fn set_maximize(&self, maximize: bool) {
        self.inner.set_maximize(maximize);
    }
    fn set_minimize(&self, minimize: bool) {
        self.inner.set_minimize(minimize);
    }
    fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }
    fn set_undecorated(&self, undecorated: bool) {
        self.inner.set_undecorated(undecorated);
    }
}
