use std::ops::Range;

use specs::{Component, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct MeshRenderer {
    pub vertex_buffer: Option<wgpu::Buffer>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Range<u32>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
