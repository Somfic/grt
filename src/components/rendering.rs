use std::ops::Range;

use specs::{Component, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Renderer {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub indices_count: u32,

    pub diffuse: Option<wgpu::BindGroup>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Material {
    pub diffuse_texture: String,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
