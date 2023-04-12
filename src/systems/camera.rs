use specs::Join;
use wgpu::util::DeviceExt;

use crate::{
    components::rendering::{Camera, CameraUniform, Renderer, Transform},
    material_manager::MaterialManager,
};

pub struct CameraSystem;

impl<'a> specs::System<'a> for CameraSystem {
    type SystemData = (
        specs::ReadStorage<'a, Camera>,
        specs::WriteStorage<'a, Transform>,
        specs::ReadStorage<'a, Renderer>,
        specs::ReadExpect<'a, wgpu::Device>,
        specs::ReadExpect<'a, MaterialManager>,
    );

    fn run(
        &mut self,
        (cameras, mut transforms, renderers, device, material_manager): Self::SystemData,
    ) {
        for (camera, transform) in (&cameras, &mut transforms).join() {
            let view_projection =
                cgmath::Matrix4::look_at_rh(transform.position, camera.target, camera.up);
            let perspective_projection = cgmath::perspective(
                cgmath::Deg(camera.fovy),
                camera.aspect,
                camera.znear,
                camera.zfar,
            );

            for (_, transform) in (&renderers, &mut transforms).join() {
                let model_projection = cgmath::Matrix4::from_translation(cgmath::Vector3 {
                    x: transform.position.x,
                    y: transform.position.y,
                    z: transform.position.z,
                }) * cgmath::Matrix4::from(transform.rotation)
                    * cgmath::Matrix4::from_nonuniform_scale(
                        transform.scale.x,
                        transform.scale.y,
                        transform.scale.z,
                    );

                let projection = OPENGL_TO_WGPU_MATRIX
                    * perspective_projection
                    * view_projection
                    * model_projection;

                let mut camera_uniform = CameraUniform::new();
                camera_uniform.set_projection(projection);

                let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Buffer"),
                    contents: bytemuck::cast_slice(&[camera_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                transform.bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: material_manager.get_camera_bind_group_layout(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }],
                    label: Some("transform_bind_group"),
                }));
            }

            // TODO: Support multiple cameras
            // This currenly breaks the loop after the first camera
            break;
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
