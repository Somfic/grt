use cgmath::Vector3;
use specs::{
    shred::Fetch, storage::MaskedStorage, Read, ReadExpect, ReadStorage, Storage, WriteStorage,
};
use wgpu::{util::DeviceExt, BindGroup};

use crate::{
    components::rendering::{Camera, CameraUniform, Mesh, Renderer, Transform},
    material_manager::{self, MaterialManager},
    systems::camera,
};

pub struct RenderSystem;

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (
        ReadStorage<'a, Renderer>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, wgpu::Surface>,
        ReadExpect<'a, wgpu::Device>,
        ReadExpect<'a, wgpu::Queue>,
        ReadExpect<'a, MaterialManager>,
    );

    fn run(
        &mut self,
        (renderers, transforms, surface, device, queue, material_manager): Self::SystemData,
    ) {
        let output = surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        use specs::Join;
        for (renderer, transform) in (&renderers, &transforms).join() {
            let shader = material_manager.get_shader("default");
            render_pass.set_pipeline(&shader.pipeline);

            render_pass.set_bind_group(0, renderer.diffuse.as_ref().unwrap(), &[]);
            render_pass.set_bind_group(1, transform.bind.as_ref().unwrap(), &[]);

            render_pass.set_vertex_buffer(0, renderer.vertex_buffer.as_ref().unwrap().slice(..));
            render_pass.set_index_buffer(
                renderer.index_buffer.as_ref().unwrap().slice(..),
                wgpu::IndexFormat::Uint16,
            );

            render_pass.draw_indexed(0..renderer.indices_count, 0, 0..1);
        }

        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
