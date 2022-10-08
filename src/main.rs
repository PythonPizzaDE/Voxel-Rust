extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};
use std::{os::raw::*, mem};

mod ogl;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(1280, 720, "Voxel", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_resizable(false);

    gl::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    let vertices = vec![
        -0.5f32, -0.5f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32,
         0.5f32, -0.5f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32,
         0.0f32,  0.5f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
    ];

    let vao = ogl::vao::VAO::new();
    vao.bind();

    let vbo = ogl::vbo::VBO::new(vertices);
    vbo.bind();

    let mut shader = ogl::shader::Shader::new("shader/vertex.glsl", "shader/fragment.glsl");
    shader.bind();
    shader.create_uniform("rotation");

    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<gl::types::GLfloat>()) as i32, 0 as *const c_void);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (6 * mem::size_of::<gl::types::GLfloat>()) as i32, (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(0);
        gl::EnableVertexAttribArray(1);
    }

    let mut t = 0;

    while !window.should_close() {
        t += 1;
        let mat1 = cgmath::Matrix4::from_angle_x(cgmath::Rad::from(cgmath::Deg(t as f32)));
        let mat2 = cgmath::Matrix4::from_angle_y(cgmath::Rad::from(cgmath::Deg(t as f32)));
        let mat3 = cgmath::Matrix4::from_angle_z(cgmath::Rad::from(cgmath::Deg(t as f32)));
        let mat4 = mat1 * mat2 * mat3;
        shader.set_matrix4_uniform("rotation", &mat4);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1f32, 0.1f32, 0.1f32, 1.0f32);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {},
            }
        }
    }
}
