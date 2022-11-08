use crate::ogl;

use super::{chunk::Chunk, config};

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut chunks: Vec<Chunk> = Vec::new();

        for x in 0..config::WORLD_SIZE_X_CHUNKS {
            for y in 0..config::WORLD_SIZE_Y_CHUNKS {
                let mut chunk = Chunk::new(x, y);
                chunk.generate_chunk();
                chunk.generate_vbo();
                chunks.push(chunk);
            }
        }
        Self {
            chunks
        }
    }

    pub fn render(&self, camera: &super::camera::Camera, shader: &ogl::shader::Shader) {
        for chunk in self.chunks.iter() {
            let x_diff = (camera.position.x - (chunk.x * config::CHUNK_WIDTH) as f32).abs();
            let y_diff = (camera.position.z - (chunk.y * config::CHUNK_DEPTH) as f32).abs();
            let distance = (x_diff*x_diff + y_diff*y_diff).sqrt() / ((config::CHUNK_WIDTH + config::CHUNK_DEPTH) / 2) as f32;
            if distance <= config::RENDER_DISTANCE {
                chunk.render(shader)
            }
        }
    }

    pub fn get_chunk(&self, x: u32, y: u32) -> &Chunk {
        self.chunks.get((config::WORLD_SIZE_Y_CHUNKS * x + y) as usize).unwrap()
    }
}
