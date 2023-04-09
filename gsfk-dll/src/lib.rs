use std::ffi::{c_char, CStr};
use gsfk::API;
use gsfk::api::vulkan::Vulkan;
use gsfk::window::Window;

#[repr(C)]
pub struct VulkanAPI {
    api: API<Vulkan>
}

#[no_mangle]
pub extern "C" fn gsfkCreateWindowWithVulkan(title: *const c_char, width: u32, height: u32, mut api: &mut VulkanAPI) -> *mut Window {
    let (win, mut raw_api) = Window::new_with_vulkan(unsafe { CStr::from_ptr(title).to_str().unwrap() }, width, height);
    let win = Box::new(win);

    let mut api = &mut *api;
    api.api = raw_api;

    Box::into_raw(win)
}