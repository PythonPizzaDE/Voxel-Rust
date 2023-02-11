use crate::ogl;
use super::{block, config};
use crate::game::config::{CHUNK_WIDTH, CHUNK_DEPTH, CHUNK_HEIGHT};
use std::{os::raw::*, mem};
use perlin2d::PerlinNoise2D;

#[derive(Debug, PartialEq)]
pub struct Chunk {
    pub x: u32,
    pub y: u32,
    pub blocks: Box<Vec<block::BlockType>>,
    vbo: Option<ogl::vbo::VBO>,
    vao: ogl::vao::VAO,
    model_matrix: cgmath::Matrix4<f32>,
}

impl Chunk {
    pub fn new(x: u32, y: u32, shader: &ogl::shader::Shader) -> Self {
        let mut blocks: Box<Vec<block::BlockType>> = Box::new(Vec::new());
        for _ in 0..(CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) {
            blocks.push(block::BlockType::Air);
        }

        let vao = ogl::vao::VAO::new();
        vao.bind();

        shader.bind();



        let model_matrix = cgmath::Matrix4::from_translation(cgmath::vec3((x * CHUNK_WIDTH) as f32, 0f32, (y * CHUNK_DEPTH) as f32));

        Self {
            x,
            y,
            blocks,
            vbo: None,
            vao,
            model_matrix,
        }
    }

    pub fn set_block(&mut self, x: u32, y: u32, z: u32, block_type: block::BlockType) {
        self.blocks[(((z * (CHUNK_WIDTH) * (CHUNK_HEIGHT)) + (y * (CHUNK_WIDTH))) + x) as usize] = block_type;
    }

    pub fn get_block(&mut self, x: u32, y: u32, z: u32) -> block::BlockType {
        self.blocks[(((z * (CHUNK_WIDTH) * (CHUNK_HEIGHT)) + (y * (CHUNK_WIDTH))) + x) as usize]
    }

    pub fn generate_chunk(&mut self, perlin: &PerlinNoise2D) {
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_DEPTH {
                for y in 0..CHUNK_HEIGHT {
                    let height = perlin.get_noise(((config::CHUNK_WIDTH * self.x) + x) as f64, ((config::CHUNK_DEPTH * self.y) + z) as f64);
                    if (y as f64) < height - 1f64 {
                        self.set_block(x, y, z, block::BlockType::Stone)
                    } else if (y as f64) <= height {
                        self.set_block(x, y, z, block::BlockType::Grass)
                    }
                }
            }
        }
    }

    pub fn generate_vbo(&mut self) {
        let mut vertices: Vec<f32> = Vec::new();
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let current_block = self.get_block(x, y, z);
                    if current_block != block::BlockType::Air {
                        let mut block_mesh = block::BlockVerticesBuilder::new();

                        if x == CHUNK_WIDTH - 1 || (self.get_block(x + 1, y, z) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::East, current_block, x, y, z);
                        } 
                        if x == 0 || (self.get_block(x - 1, y, z) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::West, current_block, x, y, z);
                        }

                        if y == CHUNK_HEIGHT - 1 || (self.get_block(x, y + 1, z) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::Top, current_block, x, y, z);
                        }
                        if y == 0 || (self.get_block(x, y - 1, z) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::Bottom, current_block, x, y, z);
                        }

                        if z == CHUNK_DEPTH - 1 || (self.get_block(x, y, z + 1) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::North, current_block, x, y, z);
                        }
                        if z == 0 || (self.get_block(x, y, z - 1) == block::BlockType::Air) {
                            block_mesh.add(block::BlockFace::South, current_block, x, y, z);
                        }

                        vertices.extend(block_mesh.build());
                    }
                }
            }
        }

        self.vao.bind();

        let vbo = ogl::vbo::VBO::new(vertices);
        vbo.bind();

        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, 0 as *const c_void);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, (3 * mem::size_of::<f32>()) as *const c_void);
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, (9 * mem::size_of::<f32>()) as i32, (6 * mem::size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
        }

        self.vbo = Some(vbo);
    }

    pub fn render(&self, shader: &ogl::shader::Shader) {
        self.vao.bind();
        shader.bind();
        shader.set_matrix4_uniform("model", &self.model_matrix);
        let render_vbo = self.vbo.clone().unwrap();
        render_vbo.render();
    }
}
