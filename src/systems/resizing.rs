use crate::components::rendering::Camera;
use specs::{Join, ReadExpect, WriteExpect, WriteStorage};

pub struct ResizingSystem;

impl<'a> specs::System<'a> for ResizingSystem {
    type SystemData = (
        WriteStorage<'a, Camera>,
        ReadExpect<'a, wgpu::Surface>,
        ReadExpect<'a, wgpu::Device>,
        WriteExpect<'a, wgpu::SurfaceConfiguration>,
        ReadExpect<'a, winit::dpi::PhysicalSize<u32>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, surface, device, mut config, size) = data;

        if size.width == config.width && size.height == config.height {
            return;
        }

        config.width = size.width;
        config.height = size.height;

        for camera in (&mut camera).join() {
            camera.aspect = size.width as f32 / size.height as f32;
        }

        surface.configure(&device, &config);
    }
}
