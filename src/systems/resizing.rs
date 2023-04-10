use specs::{ReadExpect, WriteExpect};

pub struct ResizingSystem;

impl<'a> specs::System<'a> for ResizingSystem {
    type SystemData = (
        ReadExpect<'a, wgpu::Surface>,
        ReadExpect<'a, wgpu::Device>,
        WriteExpect<'a, wgpu::SurfaceConfiguration>,
        ReadExpect<'a, winit::dpi::PhysicalSize<u32>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (surface, device, mut config, size) = data;

        if size.width == config.width && size.height == config.height {
            return;
        }

        config.width = size.width;
        config.height = size.height;
        surface.configure(&device, &config);
    }
}
