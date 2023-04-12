use wgpu::util::DeviceExt;

use crate::{
    components::rendering::{Material, Mesh, Model, Renderer, Vertex},
    material_manager::MaterialManager,
};

pub struct ModelBuilderSystem;

impl<'a> specs::System<'a> for ModelBuilderSystem {
    type SystemData = (
        specs::ReadStorage<'a, Model>,
        specs::WriteStorage<'a, Renderer>,
        specs::WriteExpect<'a, MaterialManager>,
        specs::ReadExpect<'a, wgpu::Device>,
        specs::ReadExpect<'a, wgpu::Queue>,
    );

    fn run(
        &mut self,
        (models, mut renderers, mut material_manager, device, queue): Self::SystemData,
    ) {
        use specs::Join;
        for (model, renderer) in (&models, &mut renderers).join() {
            // TODO: Find a way to check if the model has changed
            let needs_update = renderer.meshes.is_empty() || renderer.materials.is_empty();

            if !needs_update {
                continue;
            }

            let object_text = std::fs::read_to_string(&model.file).unwrap();
            let object_cursor = std::io::Cursor::new(object_text);
            let mut object_reader = std::io::BufReader::new(object_cursor);

            let (imported_meshes, imported_materials) = tobj::load_obj_buf(
                &mut object_reader,
                &tobj::LoadOptions {
                    triangulate: true,
                    single_index: true,
                    ..Default::default()
                },
                |p| {
                    let material_text = std::fs::read_to_string(&p).unwrap();
                    let material_cursor = std::io::Cursor::new(material_text);
                    let mut material_reader = std::io::BufReader::new(material_cursor);

                    tobj::load_mtl_buf(&mut material_reader)
                },
            )
            .unwrap(); // FIXME: Handle errors

            let mut materials = Vec::new();

            for imported_material in imported_materials.unwrap().iter() {
                let diffuse_texture = material_manager.add_texture_from_path(
                    &imported_material.diffuse_texture.to_string(),
                    &device,
                    &queue,
                );

                let bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    label: None,
                }));

                let material = Material {
                    name: imported_material.name.to_string(),
                    diffuse_bind: bind,
                };

                materials.push(material);
            }

            let meshes = imported_meshes
                .into_iter()
                .map(|m| {
                    let vertices = (0..m.mesh.positions.len() / 3)
                        .map(|i| Vertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]],
                            normal: [
                                m.mesh.normals[i * 3],
                                m.mesh.normals[i * 3 + 1],
                                m.mesh.normals[i * 3 + 2],
                            ],
                        })
                        .collect::<Vec<_>>();

                    let vertex_buffer = Some(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(&format!("{:?} Vertex Buffer", model.file)),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX,
                        },
                    ));
                    let index_buffer = Some(device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some(&format!("{:?} Index Buffer", model.file)),
                            contents: bytemuck::cast_slice(&m.mesh.indices),
                            usage: wgpu::BufferUsages::INDEX,
                        },
                    ));

                    Mesh {
                        name: model.file.to_string(),
                        vertex_buffer,
                        index_buffer,
                        num_elements: m.mesh.indices.len() as u32,
                        material: m.mesh.material_id.unwrap_or(0),
                    }
                })
                .collect::<Vec<_>>();

            log::info!(
                "Loaded model: {:?} with {} meshes and {} materials",
                model.file,
                meshes.len(),
                materials.len()
            );

            renderer.materials = materials;
            renderer.meshes = meshes;
        }
    }
}
