extern crate gl;

use std::{os::raw::*, mem};

#[derive(Clone, Debug, PartialEq)]
pub struct VBO  {
    id: u32,
    vertices: Vec<f32>,
}

impl VBO {
    pub fn new(data: Vec<f32>) -> Self { 
        let mut vbo_id: u32 = 0;
         unsafe {
            gl::CreateBuffers(1, &mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (data.len() * mem::size_of::<gl::types::GLfloat>()) as isize, &data[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
        };

        Self {
            id: vbo_id,
            vertices: data,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn render(&self) {
        self.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}
