use std::cmp::Ordering;

use glam::Vec3;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vec3,
    tex_coords: [f32; 2],
}

impl Vertex {
    pub const fn zero() -> Self {
        Self {
            position: Vec3::ZERO,
            tex_coords: [0.0; 2],
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

const BLOCK_SIZE: f32 = 2.0;
const VERTICES_PER_BLOCK: usize = 24;
const INDICES_PER_BLOCK: usize = 36;

// Block locations are stored by their position within a chunk
#[derive(Clone, Copy)]
struct Block {
    block_num: u16,

}

impl Block {
    const fn new(block_num: u16) -> Self {
        Self { 
            block_num,

        }

    }

    fn as_vertices_top(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, 1.0]), tex_coords: [1.0, 1.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, 1.0]), tex_coords: [1.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, 1.0]), tex_coords: [0.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, 1.0]), tex_coords: [0.0, 1.0] },
        ]
    }

    fn as_vertices_bottom(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, -1.0]), tex_coords: [1.0, 1.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, -1.0]), tex_coords: [1.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, -1.0]), tex_coords: [0.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, -1.0]), tex_coords: [0.0, 1.0] },
        ]
    }

    fn as_vertices_left(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, 1.0]), tex_coords: [1.0, 1.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, 1.0]), tex_coords: [1.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, -1.0]), tex_coords: [0.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, -1.0]), tex_coords: [0.0, 1.0] },
        ]
    }

    fn as_vertices_right(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, -1.0]), tex_coords: [1.0, 1.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, -1.0]), tex_coords: [1.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, 1.0]), tex_coords: [0.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, 1.0]), tex_coords: [0.0, 1.0] },
        ]
    }

    fn as_vertices_front(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, -1.0]), tex_coords: [0.0, 0.0], }, // Top left
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, -1.0]), tex_coords: [0.0, 1.0], }, // Bottom left 
            Vertex { position: block_pos + Vec3::from_array([-1.0, 1.0, 1.0]), tex_coords: [1.0, 1.0], }, // Bottom right 
            Vertex { position: block_pos + Vec3::from_array([1.0, 1.0, 1.0]), tex_coords: [1.0, 0.0], }, // Top right 
        ] 
    }

    fn as_vertices_back(&self, block_pos: Vec3) -> [Vertex; 4] {
        [
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, 1.0]), tex_coords: [1.0, 1.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, 1.0]), tex_coords: [1.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([-1.0, -1.0, -1.0]), tex_coords: [0.0, 0.0] },
            Vertex { position: block_pos + Vec3::from_array([1.0, -1.0, -1.0]), tex_coords: [0.0, 1.0] },
        ]
    }

    const fn as_indices(block_num: u16) -> [u16; INDICES_PER_BLOCK] {
        let block_start_index = block_num * 24;

        [
            block_start_index, 1 + block_start_index, 2 + block_start_index, 2 + block_start_index, 3 + block_start_index, block_start_index, // top
            4 + block_start_index, 5 + block_start_index, 6 + block_start_index, 6 + block_start_index, 7 + block_start_index, 4 + block_start_index, // bottom
            8 + block_start_index, 9 + block_start_index, 10 + block_start_index, 10 + block_start_index, 11 + block_start_index, 8 + block_start_index, // right
            12 + block_start_index, 13 + block_start_index, 14 + block_start_index, 14 + block_start_index, 15 + block_start_index, 12 + block_start_index, // left
            16 + block_start_index, 17 + block_start_index, 18 + block_start_index, 18 + block_start_index, 19 + block_start_index, 16 + block_start_index, // front
            20 + block_start_index, 21 + block_start_index, 22 + block_start_index, 22 + block_start_index, 23 + block_start_index, 20 + block_start_index, // back           
        ]
    }

    fn calc_rel_pos(block_num: u16) -> Vec3 {
        let z = ((block_num as f32) / (CHUNK_SIZE_AXIS.pow(2) as f32)).ceil(); 
        let y = (((block_num as f32) - z * CHUNK_SIZE_AXIS.pow(2) as f32) / CHUNK_SIZE_AXIS as f32).ceil();
        let x = ((block_num as f32) - CHUNK_SIZE_AXIS as f32 * (y + CHUNK_SIZE_AXIS as f32 * z)).ceil();

        Vec3::new(x, y, z) * Vec3::splat(2.0)
    }

}

// The length a chunk goes on a single axis
const CHUNK_SIZE_AXIS: usize = 8;
pub const CHUNK_SIZE: usize = CHUNK_SIZE_AXIS.pow(3);

pub struct Chunk {
    chunk_pos: Vec3,
    blocks: [Block; CHUNK_SIZE],
    vertices: Vec<Vertex>,
    indices: Vec<u16>,

}

impl Chunk {
    pub fn new(chunk_pos: Vec3) -> Self {
        let blocks = [Block::new(0); CHUNK_SIZE];
        
        let mut old_self = Self {
            chunk_pos,
            blocks,
            vertices: Vec::with_capacity(CHUNK_SIZE * VERTICES_PER_BLOCK),
            indices: Vec::with_capacity(CHUNK_SIZE * INDICES_PER_BLOCK),

        };
        
        old_self.update_vertices(Vec3::ZERO);
        old_self.update_indices();

        old_self

    }

    pub fn update_graphics(&mut self, camera_pos: Vec3) {
        self.update_block_nums();
        self.sort_blocks(camera_pos);
        self.update_vertices(camera_pos);
        self.update_indices();

    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices

    }

    pub fn indices(&self) -> &[u16] {
        &self.indices

    }

    /// Sort the blocks by their distance to the player
    fn sort_blocks(&mut self, camera_pos: Vec3) {
        self.blocks.sort_unstable_by(|block1, block2| {
            let pos1 = Block::calc_rel_pos(block1.block_num) + self.chunk_pos;
            let pos2 = Block::calc_rel_pos(block2.block_num) + self.chunk_pos;

            let distance1 = pos1.distance_squared(camera_pos);
            let distance2 = pos2.distance_squared(camera_pos);

            distance2.partial_cmp(&distance1).unwrap_or(Ordering::Equal)

        });

    }

    fn update_block_nums(&mut self) {
        self.blocks.iter_mut().enumerate().for_each(|(block_num, block)| block.block_num = (block_num + 1).try_into().unwrap());

    }

    fn update_vertices(&mut self, camera_pos: Vec3) {
        self.vertices.clear();

        for block in self.blocks.iter() {
            // The block's position relative to the chunk center
            let block_rel_pos = Block::calc_rel_pos(block.block_num); 
            let block_world_pos = self.chunk_pos + block_rel_pos;

            let mut current_block_vertices = [Vertex::zero(); VERTICES_PER_BLOCK];

            current_block_vertices[8..12].copy_from_slice(&block.as_vertices_right(block_world_pos));
            current_block_vertices[12..16].copy_from_slice(&block.as_vertices_left(block_world_pos));
            current_block_vertices[16..20].copy_from_slice(&block.as_vertices_front(block_world_pos));
            current_block_vertices[20..24].copy_from_slice(&block.as_vertices_back(block_world_pos));
            current_block_vertices[4..8].copy_from_slice(&block.as_vertices_bottom(block_world_pos));
            current_block_vertices[0..4].copy_from_slice(&block.as_vertices_top(block_world_pos));

            self.vertices.extend_from_slice(&current_block_vertices);

        }

    }

    fn update_indices(&mut self) {
        self.indices.clear();

        for i in 0..self.blocks.len() {  
            let current_block_indices = Block::as_indices(i.try_into().unwrap());
            self.indices.extend_from_slice(&current_block_indices);

        }

    }

}
