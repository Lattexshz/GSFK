mod ffi;

use gsfk::api::gl::{OpenGL, OpenGLAPIDescription, OpenGLAPIExt};
use gsfk::api::vulkan::Vulkan;
use gsfk::window::Window;
use gsfk::{WindowEvent, WindowImplementation};
use gsfk::API;
use std::ffi::{c_char, c_void, CStr};
use std::sync::{LockResult, Mutex};
use once_cell::sync::OnceCell;


#[repr(C)]
pub struct VulkanAPI {
    api: API<Vulkan>,
}

#[repr(C)]
pub struct OpenGLAPI {
    api: API<OpenGL>,
}

#[no_mangle]
pub extern "C" fn gsfkCreateWindowWithOpenGL(
    title: *const c_char,
    width: u32,
    height: u32,
    api: *mut OpenGLAPI,
) -> *mut Window {
    let (win, raw_api) = Window::new_with_opengl(
        unsafe { CStr::from_ptr(title).to_str().unwrap() },
        width,
        height,
        OpenGLAPIDescription {
            version_major: 4,
            version_minor: 6,
        },
    );
    let win = Box::new(win);

    let mut api = unsafe { &mut *api };
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
pub extern "C" fn gsfkCreateWindowWithVulkan(
    title: *const c_char,
    width: u32,
    height: u32,
    api: *mut VulkanAPI,
) -> *mut Window {
    let (win, raw_api) = Window::new_with_vulkan(
        unsafe { CStr::from_ptr(title).to_str().unwrap() },
        width,
        height,
    );
    let win = Box::new(win);

    let mut api = unsafe { &mut *api };
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
pub extern "C" fn gsfkShowWindow(window: *mut Window) {
    let window = ref_from_ptr!(window);
    window.show();
}

#[no_mangle]
pub extern "C" fn gsfkHideWindow(window: *mut Window) {
    let window = ref_from_ptr!(window);
    window.hide();
}

#[no_mangle]
pub extern "C" fn gsfkSetMaximizeWindow(window: *mut Window, b: u8) {
    let maximize = BOOL!(b);
    let window = ref_from_ptr!(window);
    window.set_maximize(maximize);
}

#[no_mangle]
pub extern "C" fn gsfkSetMinimizeWindow(window: *mut Window, b: u8) {
    let minimize = BOOL!(b);
    let window = ref_from_ptr!(window);
    window.set_minimize(minimize);
}

#[no_mangle]
pub extern "C" fn gsfkSetWindowTitle(window: *mut Window, title: *const c_char) {
    let window = ref_from_ptr!(window);
    let title = cstr_to_str!(title);
    window.set_title(title);
}

#[no_mangle]
pub extern "C" fn gsfkSetWindowUndecorated(window: *mut Window, undecorated: u8) {
    let window = ref_from_ptr!(window);
    let undecorated = BOOL!(undecorated);
    window.set_undecorated(undecorated);
}

#[no_mangle]
pub extern "C" fn gsfkRunWindow(window: *mut Window) {
    let window = ref_from_ptr!(window);
    window.run(|event,control_flow| {
        match event {
            WindowEvent::Update => {
                match UPDATE_REQUESTED.lock() {
                    Ok(s) => {
                        match s.get() {
                            None => {}
                            Some(c) => {
                                c();
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            WindowEvent::KeyDown(_) => {}
            WindowEvent::KeyUp(_) => {}
            WindowEvent::RedrawRequested => {
                println!("Redraw");
                match REDRAW_REQUESTED.lock().unwrap().get() {
                    None => {
                        println!("None");
                    }
                    Some(c) => {
                        c();
                    }
                }
            }
            WindowEvent::CloseRequested => {
                match CLOSE_REQUESTED.lock().unwrap().get() {
                    None => {}
                    Some(c) => {
                        c();
                    }
                }
            }
        }
    })
}

// OpenGL API processes
#[no_mangle]
pub extern "C" fn gsfkGLMakeCurrent(gl: *mut OpenGLAPI) {
    let gl = ref_from_ptr!(gl);
    gl.api.get_api().make_current();
}

#[no_mangle]
pub extern "C" fn gsfkGLSwapInterval(gl: *mut OpenGLAPI, bool: u32) {
    let gl = ref_from_ptr!(gl);

    gl.api.get_api().swap_interval(bool != 0);
}

#[no_mangle]
pub extern "C" fn gsfkGLGetProcAddress(gl: *mut OpenGLAPI, addr: *const c_char) -> *const c_void {
    let gl = ref_from_ptr!(gl);
    let addr = unsafe { CStr::from_ptr(addr) }.to_str().unwrap();

    gl.api.get_api().get_proc_address(addr)
}

#[no_mangle]
pub extern "C" fn gsfkGLSwapBuffers(gl: *mut OpenGLAPI) {
    let gl = ref_from_ptr!(gl);
    gl.api.get_api().swap_buffers();
}

// Callbacks
pub type UPDATECALLBACK = extern "C" fn();
pub type REDRAWREQUESTEDCALLBACK = extern "C" fn();
pub type CLOSEREQUESTEDCALLBACK = extern "C" fn();

static UPDATE_REQUESTED:Mutex<OnceCell<UPDATECALLBACK>> = Mutex::new(OnceCell::new());
static REDRAW_REQUESTED:Mutex<OnceCell<REDRAWREQUESTEDCALLBACK>> = Mutex::new(OnceCell::new());
static CLOSE_REQUESTED:Mutex<OnceCell<CLOSEREQUESTEDCALLBACK>> = Mutex::new(OnceCell::new());

#[no_mangle]
pub extern "C" fn gsfkSetUpdatedCallback(callback: UPDATECALLBACK) {
    REDRAW_REQUESTED.lock().unwrap().set(callback).unwrap();
}

#[no_mangle]
pub extern "C" fn gsfkSetRedrawRequestedCallback(callback: REDRAWREQUESTEDCALLBACK) {
    REDRAW_REQUESTED.lock().unwrap().set(callback).unwrap();
}

#[no_mangle]
pub extern "C" fn gsfkSetCloseRequestedCallback(callback: CLOSEREQUESTEDCALLBACK) {
    REDRAW_REQUESTED.lock().unwrap().set(callback).unwrap();
}