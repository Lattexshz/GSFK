pub use winey::WineyWindowImplementation as WindowImplementation;

pub mod api;
pub mod platform;
pub mod window;
pub mod keyboard;

pub type WindowEvent = winey::WindowEvent;
pub type ControlFlow = winey::window::ControlFlow;

pub struct API<T> {
    context: T,
}

impl<T> API<T> {
    pub fn get_api(&self) -> &T {
        &self.context
    }
}

pub struct Version(pub u32, pub u32, pub u32);

pub struct APIDescription {
    pub version: Version,
}
