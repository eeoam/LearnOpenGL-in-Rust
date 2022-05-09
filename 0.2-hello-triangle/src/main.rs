

extern crate sdl2;
extern crate gl;

/* use std; */
use std::ffi::{CString};




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


    

let vert_shader_src = CString::new(include_str!("triangle.vert")).unwrap();
let vert_shader = unsafe { gl::CreateShader (gl::VERTEX_SHADER) };
unsafe {
    gl::ShaderSource(vert_shader, 1, &vert_shader_src.as_ptr(), std::ptr::null());
    gl::CompileShader(vert_shader);
}

let mut success: gl::types::GLint = 1;
unsafe {
    gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
}
if success == 0 {
    
let mut len: gl::types::GLint = 0;
unsafe {
    gl::GetShaderiv(vert_shader, gl::INFO_LOG_LENGTH, &mut len);
}
let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
buffer.extend([b' '].iter().cycle().take(len as usize));
let mut error = unsafe { CString::from_vec_unchecked(buffer) };


    unsafe {
        gl::GetShaderInfoLog(
            vert_shader,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );
    }

    println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n");
}





let vertices: Vec<f32>
    = vec![-0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0];

let mut vao: gl::types::GLuint = 0;
unsafe { gl::GenVertexArrays(1, &mut vao); }

let mut vbo: gl::types::GLuint = 0;
unsafe { gl::GenBuffers(1, &mut vbo); }

unsafe {
    gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(0); // layout (location = 0) in vertex shader

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                std::ptr::null()
            );


        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);
}



let frag_shader_src = CString::new(include_str!("triangle.frag")).unwrap();
let frag_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
unsafe {
    gl::ShaderSource(frag_shader, 1, &frag_shader_src.as_ptr(), std::ptr::null());
    gl::CompileShader(frag_shader);
}

success = 1;
unsafe {
    gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success);
}
if success == 0 {
    
let mut len: gl::types::GLint = 0;
unsafe {
    gl::GetShaderiv(vert_shader, gl::INFO_LOG_LENGTH, &mut len);
}
let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
buffer.extend([b' '].iter().cycle().take(len as usize));
let mut error = unsafe { CString::from_vec_unchecked(buffer) };


    unsafe {
        gl::GetShaderInfoLog(
            frag_shader,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
        );
    }

    println!("{}", error.into_string().unwrap());
}





let program_id = unsafe { gl::CreateProgram() };

unsafe {
    gl::AttachShader(program_id, vert_shader);
    gl::AttachShader(program_id, frag_shader);
    gl::LinkProgram(program_id);
}


unsafe {
    gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
}
if success == 0 {
    
let mut len: gl::types::GLint = 0;
unsafe {
    gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
}
let mut buffer = Vec::with_capacity(len as usize + 1);
buffer.extend([b' '].iter().cycle().take(len as usize));
let error = unsafe { CString::from_vec_unchecked(buffer) };


    unsafe {
        gl::GetProgramInfoLog(
            program_id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar
        );
    }
    println!("{}", error.into_string().unwrap());
}



unsafe {
    gl::DetachShader(program_id, vert_shader);
    gl::DetachShader(program_id, frag_shader);
}




    
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

        unsafe {
            gl::UseProgram(program_id);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        

        window.gl_swap_window();
    }



}


