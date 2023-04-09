use std::ffi::{c_char, CStr};
use gsfk::API;
use gsfk::api::gl::{OpenGL, OpenGLAPIDescription};
use gsfk::api::vulkan::Vulkan;
use gsfk::window::Window;
use gsfk::WindowImplementation;

#[repr(C)]
pub struct VulkanAPI {
    api: API<Vulkan>
}

#[repr(C)]
pub struct OpenGLAPI {
    api: API<OpenGL>
}

#[no_mangle]
pub extern "C" fn gsfkCreateWindowWithOpenGL(title: *const c_char,width: u32,height: u32,mut api: &mut OpenGLAPI) -> *mut Window {
    let (win, mut raw_api) = Window::new_with_opengl(unsafe { CStr::from_ptr(title).to_str().unwrap() }, width, height,OpenGLAPIDescription {
        version_major: 4,
        version_minor: 6,
    });
    let win = Box::new(win);

    let mut api = &mut *api;
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
pub extern "C" fn gsfkCreateWindowWithVulkan(title: *const c_char, width: u32, height: u32, mut api: &mut VulkanAPI) -> *mut Window {
    let (win, mut raw_api) = Window::new_with_vulkan(unsafe { CStr::from_ptr(title).to_str().unwrap() }, width, height);
    let win = Box::new(win);

    let mut api = &mut *api;
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
pub extern "C" fn gsfkShowWindow(window: *mut Window) {
    let window = unsafe { &*window };
    window.show();
}

#[no_mangle]
pub extern "C" fn gsfkHideWindow(window: *mut Window) {
    let window = unsafe { &*window };
    window.hide();
}