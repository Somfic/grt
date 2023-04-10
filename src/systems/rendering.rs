use specs::{Read, ReadExpect};

pub struct RenderSystem;

impl<'a> specs::System<'a> for RenderSystem {
    type SystemData = (
        ReadExpect<'a, wgpu::Surface>,
        ReadExpect<'a, wgpu::Device>,
        ReadExpect<'a, wgpu::Queue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (surface, device, queue) = data;
        match RenderSystem::render(self, surface, device, queue) {
            Ok(_) => (),
            Err(e) => {
                // Panic
                panic!("Error rendering: {:?}", e);
            }
        }
    }

    fn setup(&mut self, world: &mut specs::World) {}
}

impl RenderSystem {
    fn render(
        &mut self,
        surface: Read<wgpu::Surface, specs::shred::PanicHandler>,
        device: Read<wgpu::Device, specs::shred::PanicHandler>,
        queue: Read<wgpu::Queue, specs::shred::PanicHandler>,
    ) -> Result<(), wgpu::SurfaceError> {
        let output = surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        drop(render_pass);

        queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
