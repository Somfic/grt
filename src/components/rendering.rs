use std::ops::Range;

use cgmath::Rotation3;
use specs::{Component, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Renderer {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub indices_count: u32,

    pub diffuse: Option<wgpu::BindGroup>,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Material {
    pub diffuse_texture: String,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn descriptor<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.projection.into();
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Camera {
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,

    pub projection: cgmath::Matrix4<f32>,
    pub uniform: Option<wgpu::Buffer>,
    pub bind: Option<wgpu::BindGroup>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: cgmath::Point3::new(0.0, 0.0, 0.0),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0,
            fovy: 45.0,
            znear: 0.001,
            zfar: 1000.0,
            projection: cgmath::Matrix4::from_scale(1 as f32),
            uniform: None,
            bind: None,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: cgmath::Point3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: cgmath::Point3::new(0.0, 0.0, 0.0),
            rotation: cgmath::Quaternion::from_axis_angle(
                cgmath::Vector3::unit_z(),
                cgmath::Deg(0.0),
            ),
            scale: cgmath::Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
