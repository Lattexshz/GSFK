use std::ffi::CStr;
use winey::WindowEvent;
use gsfk::api::gl::{GLProfile, OpenGLAPIDescription, OpenGLAPIExt};
use gsfk::keyboard::*;
use gsfk::window::Window;
use gsfk::WindowImplementation;


fn main() {
    let desc = OpenGLAPIDescription {
        version_major: 4,
        version_minor: 6,
        profile: GLProfile::ES,
    };

    let (window, opengl) = Window::new_with_opengl("OpenGL Window!", 500, 500, desc);
    let gl = opengl.get_api();
    gl.make_current();

    get_key_name(GSFK_KEY_A);

    gl::load_with(|s| gl.get_proc_address(s));

    gl.swap_interval(true);

    unsafe {
        println!("{}",CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str().unwrap());
    }

    window.show();
    window.run(|event, control_flow| unsafe {
        match event {
            WindowEvent::KeyEvent(code) => {
                println!("{}",get_key_name(code));
                if code == GSFK_KEY_TAB {
                    std::process::exit(0);
                }
            }
            WindowEvent::Update => {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl.swap_buffers();
            }
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            _ => {}
        }
    })
}
