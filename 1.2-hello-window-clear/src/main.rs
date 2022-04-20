
extern crate sdl2;
extern crate gl;

fn main () {
    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_version(4, 1);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

    let window = video_subsystem
        .window("LearnOpenGL", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                },

                sdl2::event::Event::Quit {
                    ..
                } => {
                    break 'main
                },

                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}

