#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockType {
    Air,
    Stone,
    Grass,
}

pub enum BlockFace {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

pub struct BlockVerticesBuilder {
    vertices: Vec<f32>
}

fn block_color(block_type: BlockType) -> (f32, f32, f32) {
    return match block_type {
        BlockType::Stone => (0.5f32, 0.5f32, 0.5f32),
        BlockType::Grass => (0.16f32, 0.68f32, 0.34f32),
        BlockType::Air => (0f32, 0f32, 0f32),
    }
}

impl BlockVerticesBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new()
        }
    }
    pub fn add(&mut self, face: BlockFace, block_type: BlockType, x: u32, y: u32, z: u32) -> &mut Self {
        let (r, g, b) = block_color(block_type);

        match face {
                                                    //  x                y                z                r  g  b       normals
            BlockFace::Top => self.vertices.extend(vec![1f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,
                                                        0f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,
                                                        1f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,
                                                        1f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,
                                                        0f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,
                                                        0f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 1f32, 0f32,]),

            BlockFace::Bottom => self.vertices.extend(vec![1f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,
                                                           0f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,
                                                           1f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,
                                                           1f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,
                                                           0f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,
                                                           0f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, -1f32, 0f32,]),

            BlockFace::North => self.vertices.extend(vec![1f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,
                                                          0f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,
                                                          1f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,
                                                          1f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,
                                                          0f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,
                                                          0f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 0f32, 0f32, 1f32,]),

            BlockFace::South => self.vertices.extend(vec![1f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,
                                                          0f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,
                                                          1f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,
                                                          1f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,
                                                          0f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,
                                                          0f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 0f32, 0f32, -1f32,]),

            BlockFace::East => self.vertices.extend(vec![1f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,
                                                         1f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,
                                                         1f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,
                                                         1f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,
                                                         1f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,
                                                         1f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, 1f32, 0f32, 0f32,]),

            BlockFace::West => self.vertices.extend(vec![0f32 + x as f32, 1f32 + y as f32, 1f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,
                                                         0f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,
                                                         0f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,
                                                         0f32 + x as f32, 0f32 + y as f32, 1f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,
                                                         0f32 + x as f32, 0f32 + y as f32, 0f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,
                                                         0f32 + x as f32, 1f32 + y as f32, 0f32 + z as f32, r, g, b, -1f32, 0f32, 0f32,]),
        }

        self
    }

    pub fn build(&mut self) -> Vec<f32> {
        self.vertices.clone()
    }
}
