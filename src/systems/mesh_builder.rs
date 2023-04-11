use crate::components::rendering::{Mesh, MeshRenderer};
use wgpu::util::DeviceExt;

pub struct MeshBuilderSystem;

impl<'a> specs::System<'a> for MeshBuilderSystem {
    type SystemData = (
        specs::WriteStorage<'a, Mesh>,
        specs::WriteStorage<'a, MeshRenderer>,
        specs::ReadExpect<'a, wgpu::Device>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut meshes, mut mesh_renderers, device) = data;

        use specs::Join;
        for (mesh, mesh_renderer) in (&mut meshes, &mut mesh_renderers).join() {
            let needs_update = mesh_renderer.vertex_buffer.is_none()
                || mesh_renderer.index_buffer.is_none()
                || mesh_renderer.indices_count != mesh.indices.len() as u32;

            if !needs_update {
                continue;
            }

            mesh_renderer.vertex_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&mesh.vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));

            mesh_renderer.index_buffer = Some(device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                },
            ));

            mesh_renderer.indices_count = mesh.indices.len() as u32;
        }
    }
}
