use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use winey::platform::{Margin, WindowCorner};
use winey::WindowEvent;
use winey::window::ControlFlow;

use crate::{API, APIDescription, WindowImplementation};
use crate::api::gl::{OpenGL, OpenGLAPIDescription};
use crate::api::vulkan::{Vulkan, VulkanAPIDescription};

pub type WindowRect = winey::WindowRect;
pub type WindowLevel = winey::WindowLevel;
pub type WindowType = winey::WindowType;
pub type CursorIcon = winey::CursorIcon;

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

    fn set_window_level(&self, level: WindowLevel) {
        self.inner.set_window_level(level);
    }

    fn set_window_type(&self, type_: WindowType) {
        self.inner.set_window_type(type_);
    }

    fn set_cursor_icon(&self, icon: CursorIcon) {
        self.inner.set_cursor_icon(icon);
    }

    fn get_title(&self) -> String {
        self.inner.get_title()
    }

    fn get_window_pos(&self) -> (u32, u32) {
        self.inner.get_window_pos()
    }

    fn get_window_rect(&self) -> WindowRect {
        self.inner.get_window_rect()
    }
}

#[cfg(target_os = "windows")]
impl crate::platform::WindowExtForWindows for Window {
    fn set_window_corner_radius(&self, corner: WindowCorner) {
        self.inner.set_window_corner_radius(corner);
    }

    fn set_window_border_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_border_color(r,g,b);
    }

    fn set_window_caption_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_caption_color(r,g,b)
    }

    fn set_window_text_color(&self, r: u8, g: u8, b: u8) {
        self.inner.set_window_text_color(r,g,b);
    }

    fn extend_frame_into_client_area(&self, rect: Margin) {
        self.inner.extend_frame_into_client_area(rect);
    }
}
