mod ffi;

use gsfk::api::gl::{OpenGL, OpenGLAPIDescription, OpenGLAPIExt};
use gsfk::api::vulkan::Vulkan;
use gsfk::{WindowEvent, WindowImplementation};
use gsfk::API;
use std::ffi::{c_char, c_void, CStr};
use std::sync::{LockResult, Mutex};
use once_cell::unsync::OnceCell;

pub type GSFKWindow = gsfk::window::Window;
pub type WindowPtr = *mut Window;

#[repr(C)]
pub struct Window {
    inner: GSFKWindow,
    update: UPDATECALLBACK,
    redraw: REDRAWREQUESTEDCALLBACK,
    close: CLOSEREQUESTEDCALLBACK
}

impl Window {
    pub fn new(inner: GSFKWindow) -> Self {
        Self {
            inner,
            update: def_update,
            redraw: def_redraw,
            close: def_close,
        }
    }
}

extern "C" fn def_update() {

}

extern "C" fn def_redraw() {

}

extern "C" fn def_close() {

}

#[repr(C)]
#[cfg(feature = "vulkan")]
pub struct VulkanAPI {
    api: API<Vulkan>,
}

#[repr(C)]
#[cfg(feature = "gl")]
pub struct OpenGLAPI {
    api: API<OpenGL>,
}

#[no_mangle]
#[cfg(feature = "gl")]
pub extern "C" fn gsfkCreateWindowWithOpenGL(
    title: *const c_char,
    width: u32,
    height: u32,
    api: *mut OpenGLAPI,
) -> WindowPtr {
    let (win, raw_api) = GSFKWindow::new_with_opengl(
        unsafe { CStr::from_ptr(title).to_str().unwrap() },
        width,
        height,
        OpenGLAPIDescription {
            version_major: 4,
            version_minor: 6,
        },
    );
    let win = Window::new(win);
    let win = Box::new(win);

    let mut api = unsafe { &mut *api };
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
#[cfg(feature = "vulkan")]
pub extern "C" fn gsfkCreateWindowWithVulkan(
    title: *const c_char,
    width: u32,
    height: u32,
    api: *mut VulkanAPI,
) -> WindowPtr {
    let (win, raw_api) = GSFKWindow::new_with_vulkan(
        unsafe { CStr::from_ptr(title).to_str().unwrap() },
        width,
        height,
    );
    let win = Window::new(win);
    let win = Box::new(win);

    let mut api = unsafe { &mut *api };
    api.api = raw_api;

    Box::into_raw(win)
}

#[no_mangle]
pub extern "C" fn gsfkShowWindow(window: WindowPtr) {
    let window = ref_from_ptr!(window);
    window.inner.show();
}

#[no_mangle]
pub extern "C" fn gsfkHideWindow(window: WindowPtr) {
    let window = ref_from_ptr!(window);
    window.inner.hide();
}

#[no_mangle]
pub extern "C" fn gsfkSetMaximizeWindow(window: WindowPtr, b: u8) {
    let maximize = BOOL!(b);
    let window = ref_from_ptr!(window);
    window.inner.set_maximize(maximize);
}

#[no_mangle]
pub extern "C" fn gsfkSetMinimizeWindow(window: WindowPtr, b: u8) {
    let minimize = BOOL!(b);
    let window = ref_from_ptr!(window);
    window.inner.set_minimize(minimize);
}

#[no_mangle]
pub extern "C" fn gsfkSetWindowTitle(window: WindowPtr, title: *const c_char) {
    let window = ref_from_ptr!(window);
    let title = cstr_to_str!(title);
    window.inner.set_title(title);
}

#[no_mangle]
pub extern "C" fn gsfkSetWindowUndecorated(window: WindowPtr, undecorated: u8) {
    let window = ref_from_ptr!(window);
    let undecorated = BOOL!(undecorated);
    window.inner.set_undecorated(undecorated);
}

#[no_mangle]
pub extern "C" fn gsfkRunWindow(window: WindowPtr) {
    let window = ref_from_ptr!(window);
    window.inner.run(|event,control_flow| {
        match event {
            WindowEvent::Update => {
                (window.update)();
            }
            WindowEvent::KeyDown(_) => {}
            WindowEvent::KeyUp(_) => {}
            WindowEvent::RedrawRequested => {
                (window.redraw)();
            }
            WindowEvent::CloseRequested => {
                (window.close)();
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
pub extern "C" fn gsfkSetUpdatedCallback(window: WindowPtr, mut callback: UPDATECALLBACK) {
    let window = ref_from_ptr!(window);
    window.update = callback;
}

#[no_mangle]
pub extern "C" fn gsfkSetRedrawRequestedCallback(window: WindowPtr,callback: REDRAWREQUESTEDCALLBACK) {
    let window = ref_from_ptr!(window);
    window.redraw = callback;
}

#[no_mangle]
pub extern "C" fn gsfkSetCloseRequestedCallback(window: WindowPtr,callback: CLOSEREQUESTEDCALLBACK) {
    let window = ref_from_ptr!(window);
    window.close = callback;
}

// Extensions
// Windows

#[repr(C)]
pub enum WindowCorner {
    DoNotRound,
    SmallRound,
    Round
}

impl Into<gsfk::platform::WindowCorner> for WindowCorner {
    fn into(self) -> gsfk::platform::WindowCorner {
        match self {
            WindowCorner::DoNotRound => gsfk::platform::WindowCorner::DoNotRound,
            WindowCorner::SmallRound => gsfk::platform::WindowCorner::SmallRound,
            WindowCorner::Round => gsfk::platform::WindowCorner::Round
        }
    }
}

#[repr(C)]
pub struct Margin {
    pub left_width: i32,
    pub right_width: i32,
    pub top_height: i32,
    pub bottom_height: i32,
}

impl Into<gsfk::platform::Margin> for Margin {
    fn into(self) -> gsfk::platform::Margin {
        gsfk::platform::Margin {
            left_width: self.left_width,
            right_width: self.right_width,
            top_height: self.top_height,
            bottom_height: self.bottom_height,
        }
    }
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub extern "C" fn gsfkSetWindowCornerRadius(window: *mut Window,corner: WindowCorner) {
    use gsfk::platform::WindowExtForWindows;
    let window = ref_from_ptr!(window);
    window.inner.set_window_corner_radius(corner.into());
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub extern "C" fn gsfkSetWindowCapitonColor(window: WindowPtr,r:u8,g:u8,b:u8) {
    use gsfk::platform::WindowExtForWindows;
    let window = ref_from_ptr!(window);
    window.inner.set_window_caption_color(r,g,b);
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub extern "C" fn gsfkSetWindowBorderColor(window: WindowPtr,r:u8,g:u8,b:u8) {
    use gsfk::platform::WindowExtForWindows;
    let window = ref_from_ptr!(window);
    window.inner.set_window_border_color(r,g,b);
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub extern "C" fn gsfkSetWindowTextColor(window: WindowPtr,r:u8,g:u8,b:u8) {
    use gsfk::platform::WindowExtForWindows;
    let window = ref_from_ptr!(window);
    window.inner.set_window_text_color(r,g,b);
}

#[no_mangle]
#[cfg(target_os = "windows")]
pub extern "C" fn gsfkExtendWindowFrameIntoClientArea(window: *mut Window,margin: Margin) {
    use gsfk::platform::WindowExtForWindows;
    let window = ref_from_ptr!(window);
    window.inner.extend_frame_into_client_area(margin.into());
}