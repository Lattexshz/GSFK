use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winey::WineyWindowImplementation;

use crate::{API, APIDescription};
use crate::api::vulkan::Vulkan;

pub struct Window {
    inner: winey::window::Window,
}

impl Window {
    pub fn new_with_vulkan(title: &str,width: u32,height: u32,desc: APIDescription) -> (Self,API<Vulkan>) {
        let inner = winey::window::Window::new(title,width,height);

        let api = API {
            context: Vulkan::new(inner.raw_window_handle(),desc),
        };

        (Self { inner },api)
    }
}

impl WineyWindowImplementation for Window {
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
    fn run<C: FnMut()>(&self, callback: C) {
        self.inner.run(callback);
    }
}
