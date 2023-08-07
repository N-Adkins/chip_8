extern crate sdl2;
extern crate gl;

use gl::types::*;
use sdl2::keyboard::Keycode;
use super::audio::Audio;

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {

    let shader: GLuint;
    
    unsafe {
        shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &(source.as_ptr() as *const i8), &(source.chars().count() as i32) as *const GLint);
        gl::CompileShader(shader);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {

        let mut len: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: String = if len > 0 {
            
            let mut buffer: [u8; 512] = [0; 512];
            
            unsafe {
                gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            }

            String::from_utf8_lossy(&buffer).into_owned()

        } else {
            String::new()
        };

        panic!("Failed to compile shader: {}", error);
        
    }

    shader

}

fn link_shader_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {

    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
    }

    let mut success: GLint = 1;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {

        let mut len: GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error: String = if len > 0 {
            
            let mut buffer: [u8; 512] = [0; 512];
            
            unsafe {
                gl::GetProgramInfoLog(program, 512, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut GLchar);
            }

            String::from_utf8_lossy(&buffer).into_owned()

        } else {
            String::new()
        };

        panic!("Failed to link program: {}", error);

    }
    
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    program

}

pub struct Renderer {
    pub sdl_context: sdl2::Sdl,
    pub sdl_video_subsystem: sdl2::VideoSubsystem,
    pub sdl_window: sdl2::video::Window,
    pub sdl_event_pump: sdl2::EventPump,
    pub gl_context: sdl2::video::GLContext,
    pub gl_texture: GLuint,
    pub gl_vao: GLuint,
    pub gl_shader: GLuint,
    pub gl_texture_uniform_location: GLint,
    pub audio: Audio,
    pub display: [u8; 64 * 32],
    pub keys: [u8; 0x10],
    pub last_keys: [u8; 0x10],
}

impl Renderer {
    
    pub fn new() -> Renderer { 
        
        let sdl_context = sdl2::init().unwrap();
        let sdl_video_subsystem = sdl_context.video().unwrap();
        
        let audio = Audio::new(&sdl_context, 44100, 512);

        let gl_attr = sdl_video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let sdl_window = sdl_video_subsystem
            .window("Emulator", 800, 400)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let gl_context = sdl_window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| sdl_video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        sdl_video_subsystem.gl_set_swap_interval(0).unwrap();

        let sdl_event_pump = sdl_context.event_pump().unwrap();

        let mut gl_texture: GLuint = 0;
        let mut gl_vao: GLuint = 0;
        let mut gl_vbo: GLuint = 0;
        let mut gl_ebo: GLuint = 0;

        let display: [u8; 64 * 32] = [0; 64 * 32];

        let quad_vertices: [GLfloat; 16] = [
            1.0, 1.0, 1.0, 0.0,
            1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 0.0, 1.0,
            -1.0, 1.0, 0.0, 0.0,
        ];

        let quad_indices: [GLuint; 6] = [
            0, 1, 2, 0, 2, 3,
        ];

        unsafe {
            
            gl::Viewport(0, 0, 800, 400);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0); // Set background color
            
            gl::GenTextures(1, &mut gl_texture);
            gl::BindTexture(gl::TEXTURE_2D, gl_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, 64, 32, 0, gl::RED, gl::UNSIGNED_BYTE, display.as_ptr() as *const std::os::raw::c_void);

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            
            gl::GenVertexArrays(1, &mut gl_vao);
            gl::GenBuffers(1, &mut gl_vbo);
            gl::GenBuffers(1, &mut gl_ebo);
            
            gl::BindVertexArray(gl_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, gl_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (quad_vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                &quad_vertices[0] as *const _ as *const GLvoid,
                gl::STATIC_DRAW,
            );
            
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, gl_ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (quad_indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                &quad_indices[0] as *const _ as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<GLfloat>() as GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0); 
                                                                                                                                                                                
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * std::mem::size_of::<GLfloat>() as GLsizei, (2 * std::mem::size_of::<GLfloat>()) as *const std::os::raw::c_void);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);

        }

        let vertex_shader_src = include_str!("shader/vert.glsl");
        let fragment_shader_src = include_str!("shader/frag.glsl");

        let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
        let gl_shader = link_shader_program(vertex_shader, fragment_shader);

        let gl_texture_uniform_location: GLint;

        unsafe {
            gl::UseProgram(gl_shader);
            gl_texture_uniform_location = gl::GetUniformLocation(gl_shader, "textureSampler\0".as_ptr() as *const i8);
            gl::UseProgram(0);
        }

        audio.device.resume();
        
        Renderer {
            sdl_context,
            sdl_video_subsystem,
            sdl_window,
            sdl_event_pump,
            gl_context,
            gl_texture,
            gl_vao,
            gl_shader,
            gl_texture_uniform_location,
            audio,
            display,
            last_keys: [0; 0x10],
            keys: [0; 0x10],
        }

    }

    pub fn update_texture(&mut self) {

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_texture);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, 64, 32, 0, gl::RED, gl::UNSIGNED_BYTE, self.display.as_ptr() as *const std::os::raw::c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

    }

    pub fn poll(&mut self) {
        
        self.last_keys = self.keys;

        for event in self.sdl_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => std::process::exit(0),
                sdl2::event::Event::KeyDown { keycode: Some(key), repeat, .. } => {
                    if repeat { continue; }
                    match key {
                        Keycode::Num1 => self.keys[0x1] = 1,
                        Keycode::Num2 => self.keys[0x2] = 1,
                        Keycode::Num3 => self.keys[0x3] = 1,
                        Keycode::Num4 => self.keys[0xC] = 1,
                        Keycode::Q => self.keys[0x4] = 1,
                        Keycode::W => self.keys[0x5] = 1,
                        Keycode::E => self.keys[0x6] = 1,
                        Keycode::R => self.keys[0xD] = 1,
                        Keycode::A => self.keys[0x7] = 1,
                        Keycode::S => self.keys[0x8] = 1,
                        Keycode::D => self.keys[0x9] = 1,
                        Keycode::F => self.keys[0xE] = 1,
                        Keycode::Z => self.keys[0xA] = 1,
                        Keycode::X => self.keys[0x0] = 1,
                        Keycode::C => self.keys[0xB] = 1,
                        Keycode::V => self.keys[0xF] = 1,
                        _ => (),
                    }
                },
                sdl2::event::Event::KeyUp { keycode: Some(key), repeat, .. } => {
                    if repeat { continue; }
                    match key {
                        Keycode::Num1 => self.keys[0x1] = 0,
                        Keycode::Num2 => self.keys[0x2] = 0,
                        Keycode::Num3 => self.keys[0x3] = 0,
                        Keycode::Num4 => self.keys[0xC] = 0,
                        Keycode::Q => self.keys[0x4] = 0,
                        Keycode::W => self.keys[0x5] = 0,
                        Keycode::E => self.keys[0x6] = 0,
                        Keycode::R => self.keys[0xD] = 0,
                        Keycode::A => self.keys[0x7] = 0,
                        Keycode::S => self.keys[0x8] = 0,
                        Keycode::D => self.keys[0x9] = 0,
                        Keycode::F => self.keys[0xE] = 0,
                        Keycode::Z => self.keys[0xA] = 0,
                        Keycode::X => self.keys[0x0] = 0,
                        Keycode::C => self.keys[0xB] = 0,
                        Keycode::V => self.keys[0xF] = 0,
                        _ => (),
                    }
                }
                _ => (),
            }
        } 

    }

    pub fn render(&mut self) {
        
        unsafe {

            gl::Clear(gl::COLOR_BUFFER_BIT); // Draw background
            
            gl::UseProgram(self.gl_shader);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_texture);
            gl::Uniform1i(self.gl_texture_uniform_location, 0);

            gl::BindVertexArray(self.gl_vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
            
            gl::UseProgram(0);

        }

        self.sdl_window.gl_swap_window();

    }

    pub fn clear_display(&mut self) {
        self.display.fill(0);
    }

}
