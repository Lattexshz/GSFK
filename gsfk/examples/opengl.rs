use winey::WineyWindowImplementation;
use gsfk::{API, APIDescription, Version};
use gsfk::api::vulkan::Vulkan;
use gsfk::window::Window;

fn main() {
    let desc = APIDescription {
        version: Version(4,0,0),
    };

    let (window,vulkan) = Window::new_with_vulkan("OpenGL Window!", 500, 500, desc);
    window.show();
    window.run(|| {

    })
}