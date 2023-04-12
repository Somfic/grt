use crate::{
    components::rendering::{Material, Renderer},
    material_manager::MaterialManager,
};
use wgpu::util::DeviceExt;

pub struct MaterialBuilderSystem;

impl<'a> specs::System<'a> for MaterialBuilderSystem {
    type SystemData = (
        specs::WriteStorage<'a, Material>,
        specs::WriteStorage<'a, Renderer>,
        specs::ReadExpect<'a, MaterialManager>,
        specs::ReadExpect<'a, wgpu::Device>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut materials, mut renderers, material_manager, device) = data;

        use specs::Join;
        for (material, renderer) in (&mut materials, &mut renderers).join() {
            let needs_update = renderer.diffuse.is_none();

            if !needs_update {
                continue;
            }

            let diffuse_texture = material_manager.get_texture(&material.diffuse_texture);

            let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: material_manager.get_texture_bind_group_layout(),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

            renderer.diffuse = Some(diffuse_bind_group);
        }
    }
}
