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

    pub fn render(&self, shader: &ogl::shader::Shader) {
        for chunk in self.chunks.iter() {
            chunk.render(shader)
        }
    }

    pub fn get_chunk(&self, x: u32, y: u32) -> &Chunk {
        self.chunks.get((config::WORLD_SIZE_Y_CHUNKS * x + y) as usize).unwrap()
    }
}
