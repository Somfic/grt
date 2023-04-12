use std::ops::Range;

use cgmath::Rotation3;
use specs::{Component, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Model {
    pub file: String,
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Renderer {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

#[derive(Default, Debug)]
pub struct Mesh {
    pub name: String,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_elements: u32,
    pub material: usize,
}

#[derive(Default, Debug)]
pub struct Material {
    pub name: String,
    pub diffuse_bind: Option<wgpu::BindGroup>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x3];

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

    pub fn set_projection(&mut self, projection: cgmath::Matrix4<f32>) {
        self.view_proj = projection.into();
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
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: cgmath::Point3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,

    pub bind: Option<wgpu::BindGroup>,
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
            bind: None,
        }
    }
}
