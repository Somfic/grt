use specs::Join;
use wgpu::util::DeviceExt;

use crate::components::rendering::{Camera, CameraUniform, Transform};

pub struct CameraSystem;

impl<'a> specs::System<'a> for CameraSystem {
    type SystemData = (
        specs::WriteStorage<'a, Camera>,
        specs::ReadStorage<'a, Transform>,
        specs::ReadExpect<'a, wgpu::Device>,
    );

    fn run(&mut self, (mut cameras, transforms, device): Self::SystemData) {
        for (camera, transform) in (&mut cameras, &transforms).join() {
            let view = cgmath::Matrix4::look_at_rh(transform.position, camera.target, camera.up);
            let projection = cgmath::perspective(
                cgmath::Deg(camera.fovy),
                camera.aspect,
                camera.znear,
                camera.zfar,
            );

            camera.projection = OPENGL_TO_WGPU_MATRIX * projection * view;

            let mut camera_uniform = CameraUniform::new();
            camera_uniform.update_view_proj(&camera);

            let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        }
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
