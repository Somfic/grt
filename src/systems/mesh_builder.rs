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
            if mesh_renderer.vertex_buffer.is_none() {
                mesh_renderer.vertex_buffer = Some(device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(&mesh.vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    },
                ));
            }
        }
    }
}
