use crate::{material_manager::MaterialManager, Renderer, Transform};
use specs::Join;

pub struct RenderSystem;

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (
        specs::ReadStorage<'a, Renderer>,
        specs::ReadStorage<'a, Transform>,
        specs::ReadExpect<'a, wgpu::Surface>,
        specs::ReadExpect<'a, wgpu::Device>,
        specs::ReadExpect<'a, wgpu::Queue>,
        specs::ReadExpect<'a, wgpu::SurfaceConfiguration>,
        specs::ReadExpect<'a, MaterialManager>,
    );

    fn run(
        &mut self,
        (renderers, transforms, surface, device, queue, config, material_manager): Self::SystemData,
    ) {
        let output = surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let shader = material_manager.add_shader("default", &device, &config);
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

        for (renderer, transform) in (&renderers, &transforms).join() {
            render_pass.set_pipeline(&shader.pipeline);

            render_pass.set_bind_group(
                0,
                renderer.materials[0].diffuse_bind.as_ref().unwrap(),
                &[],
            );
            render_pass.set_bind_group(1, transform.bind.as_ref().unwrap(), &[]);

            for mesh in renderer.meshes.iter() {
                render_pass.set_vertex_buffer(0, mesh.vertex_buffer.as_ref().unwrap().slice(..));
                render_pass.set_index_buffer(
                    mesh.index_buffer.as_ref().unwrap().slice(..),
                    wgpu::IndexFormat::Uint32,
                );

                render_pass.draw_indexed(0..mesh.num_elements, 0, 0..1);
            }
        }

        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
