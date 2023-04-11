use std::collections::HashMap;

use crate::components::rendering::Vertex;

pub struct ShaderManager {
    shaders: HashMap<String, ShaderInfo>,
}

pub struct ShaderInfo {
    pub shader: wgpu::ShaderModule,
    pub layout: wgpu::PipelineLayout,
    pub pipeline: wgpu::RenderPipeline,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
        }
    }

    pub fn add_shader(
        &mut self,
        name: impl Into<String>,
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) {
        let name = name.into();

        let file = format!("{name}.wgsl");
        let source = std::fs::read_to_string(file).expect("Couldn't read shader file.");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&name),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let layout_name = format!("{} Layout", name);
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&layout_name),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline_name = format!("{} Pipeline", name);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&pipeline_name),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",           // 1.
                buffers: &[Vertex::descriptor()], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });

        self.shaders.insert(
            name,
            ShaderInfo {
                shader,
                layout,
                pipeline,
            },
        );
    }

    pub fn get_shader(&self, name: &str) -> &ShaderInfo {
        self.shaders.get(name).expect("Couldn't find shader.")
    }
}
