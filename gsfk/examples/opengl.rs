use winey::WindowEvent;
use gsfk::api::gl::{OpenGLAPIDescription, OpenGLAPIExt};
use gsfk::window::Window;
use gsfk::WindowImplementation;


fn main() {
    let desc = OpenGLAPIDescription {
        version_major: 4,
        version_minor: 6,
    };

    let (window, opengl) = Window::new_with_opengl("OpenGL Window!", 500, 500, desc);
    let gl = opengl.get_api();
    gl.make_current();

    gl::load_with(|s| gl.get_proc_address(s));

    gl.swap_interval(true);

    window.show();
    window.run(|event, control_flow| unsafe {
        match event {
            WindowEvent::Update => {}
            WindowEvent::KeyDown(_) => {}
            WindowEvent::KeyUp(_) => {}
            WindowEvent::RedrawRequested => {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl.swap_buffers();
            }
            WindowEvent::CloseRequested => {
                control_flow.exit(0);
            }
        }
    })
}
