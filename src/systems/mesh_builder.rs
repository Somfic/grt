use crate::components::rendering::{Mesh, Renderer};
use wgpu::util::DeviceExt;

pub struct MeshBuilderSystem;

impl<'a> specs::System<'a> for MeshBuilderSystem {
    type SystemData = (
        specs::WriteStorage<'a, Mesh>,
        specs::WriteStorage<'a, Renderer>,
        specs::ReadExpect<'a, wgpu::Device>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut meshes, mut renderers, device) = data;

        use specs::Join;
        for (mesh, renderer) in (&mut meshes, &mut renderers).join() {
            let needs_update = renderer.vertex_buffer.is_none()
                || renderer.index_buffer.is_none()
                || renderer.indices_count != mesh.indices.len() as u32;

            if !needs_update {
                continue;
            }

            renderer.vertex_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&mesh.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));

            renderer.index_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ));

            renderer.indices_count = mesh.indices.len() as u32;
        }
    }
}
