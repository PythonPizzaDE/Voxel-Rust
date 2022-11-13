use perlin2d::PerlinNoise2D;

use crate::ogl;
use super::{chunk::Chunk, config};
use config::{CHUNK_WIDTH, WORLD_SIZE_X_CHUNKS, CHUNK_DEPTH, WORLD_SIZE_Y_CHUNKS, TERRAIN_HEIGHT_VARIATION};

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new(shader: &ogl::shader::Shader) -> Self {
        let mut chunks: Vec<Chunk> = Vec::new();
        let perlin = PerlinNoise2D::new(5, TERRAIN_HEIGHT_VARIATION, 0.5f64, 1f64, 2f64, ((CHUNK_WIDTH * WORLD_SIZE_X_CHUNKS) as f64, (CHUNK_DEPTH * WORLD_SIZE_Y_CHUNKS) as f64), 0.5f64, 1337);

        for x in 0..config::WORLD_SIZE_X_CHUNKS {
            for y in 0..config::WORLD_SIZE_Y_CHUNKS {
                chunks.push(Chunk::new(x, y, shader));
                chunks.last_mut().unwrap().generate_chunk();
                chunks.last_mut().unwrap().generate_vbo();
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
