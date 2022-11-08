use crate::ogl;
use super::block;
use crate::game::config::{CHUNK_WIDTH, CHUNK_DEPTH, CHUNK_HEIGHT};

pub struct Chunk {
    pub x: u32,
    pub y: u32,
    pub blocks: [block::BlockType; (CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) as usize],
    vbo: Option<ogl::vbo::VBO>,
    model_matrix: cgmath::Matrix4<f32>,
}

impl Chunk {
    pub fn new(x: u32, y: u32) -> Self {
        let blocks = [block::BlockType::AIR; (CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) as usize];
        let model_matrix = cgmath::Matrix4::from_translation(cgmath::vec3((x * CHUNK_WIDTH) as f32, 0f32, (y * CHUNK_DEPTH) as f32));
        Self {
            x,
            y,
            blocks,
            vbo: None,
            model_matrix,
        }
    }

    pub fn set_block(&mut self, x: u32, y: u32, z: u32, block_type: block::BlockType) {
        self.blocks[(((z * (CHUNK_WIDTH) * (CHUNK_HEIGHT)) + (y * (CHUNK_WIDTH))) + x) as usize] = block_type;
    }

    pub fn get_block(&mut self, x: u32, y: u32, z: u32) -> block::BlockType {
        self.blocks[(((z * (CHUNK_WIDTH) * (CHUNK_HEIGHT)) + (y * (CHUNK_WIDTH))) + x) as usize]
    }

    pub fn generate_chunk(&mut self) {
        self.blocks = [block::BlockType::STONE; (CHUNK_WIDTH * CHUNK_DEPTH * CHUNK_HEIGHT) as usize];
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    if y >= 20 {
                        self.set_block(x, y, z, block::BlockType::AIR)
                    }
                }
            }
        }
    }

    pub fn generate_vbo(&mut self) {
        let mut vertices: Vec<f32> = vec![];
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let block = self.get_block(x, y, z);
                    if block != block::BlockType::AIR {
                        let mut block_mesh = block::BlockVerticesBuilder::new();

                        if x == CHUNK_WIDTH - 1 || (self.get_block(x + 1, y, z) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::East, block::BlockType::STONE, x, y, z);
                        } else if x == 0 || (self.get_block(x - 1, y, z) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::West, block::BlockType::STONE, x, y, z);
                        }

                        if y == CHUNK_HEIGHT - 1 || (self.get_block(x, y + 1, z) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::Top, block::BlockType::STONE, x, y, z);
                        } else if y == 0 || (self.get_block(x, y - 1, z) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::Bottom, block::BlockType::STONE, x, y, z);
                        }

                        if z == CHUNK_DEPTH - 1 || (self.get_block(x, y, z + 1) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::North, block::BlockType::STONE, x, y, z);
                        } else if z == 0 || (self.get_block(x, y, z - 1) == block::BlockType::AIR) {
                            block_mesh.add(block::BlockFace::South, block::BlockType::STONE, x, y, z);
                        }

                        vertices.extend(block_mesh.build());
                    }
                }
            }
        }

        self.vbo = Some(ogl::vbo::VBO::new(vertices));
    }

    pub fn render(&self, shader: &ogl::shader::Shader) {
        shader.set_matrix4_uniform("model", &self.model_matrix);
        self.vbo.clone().expect("VBO has to be generated in order to render!".into()).render();
    }
}
