use cgmath::{Point3, Rotation3};
use components::rendering::{Camera, Model, Renderer, Transform};
use material_manager::MaterialManager;
use specs::{Builder, Join, WorldExt};
use systems::camera::CameraSystem;
use systems::model_builder::ModelBuilderSystem;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
mod components;
mod material_manager;
mod systems;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    initialise_logging();

    let event_loop = EventLoop::new();
    let window = create_window(&event_loop);
    let dispatcher = specs::DispatcherBuilder::new()
        .with(ModelBuilderSystem, "model_builder", &[])
        .with(CameraSystem, "camera", &[])
        .with(RotateSystem, "rotate", &[])
        .with_thread_local(crate::systems::resizing::ResizingSystem)
        .with_thread_local(crate::systems::rendering::RenderSystem)
        .build();

    let mut app = Application::new(window, dispatcher).await;

    app.world
        .create_entity()
        .with(Model {
            file: "cube.obj".to_string(),
        })
        .with(Renderer::default())
        .with(Transform::default())
        .build();

    app.world
        .create_entity()
        .with(Camera::default())
        .with(Transform {
            position: Point3 {
                x: 0.0,
                y: 0.0,
                z: 10.0,
            },
            ..Default::default()
        })
        .build();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == app.window.id() => {
            app.update();
        }

        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == app.window.id() => {
            if !app.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(physical_size) => {
                        app.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        app.resize(**new_inner_size);
                    }

                    _ => {}
                }
            }
        }

        Event::MainEventsCleared => {
            app.window.request_redraw();
        }

        _ => {}
    });
}

fn initialise_logging() {
    std::env::set_var("RUST_LOG", "warn,grt=debug");

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
}

fn create_window(window_target: &EventLoop<()>) -> winit::window::Window {
    let window = WindowBuilder::new()
        .with_title("grt")
        .build(window_target)
        .unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    window
}

struct Application {
    world: specs::World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    window: winit::window::Window,
}

impl Application {
    async fn new(
        window: winit::window::Window,
        mut dispatcher: specs::Dispatcher<'static, 'static>,
    ) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: 1,
            height: 1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let material_manager = MaterialManager::new(&device);
        material_manager.add_shader("default", &device, &config);

        let mut world = specs::World::new();

        // Resources
        world.insert(size);
        world.insert(config);
        world.insert(surface);
        world.insert(device);
        world.insert(queue);
        world.insert(material_manager);

        // Components
        world.register::<Renderer>();
        world.register::<Model>();
        world.register::<Transform>();
        world.register::<Camera>();

        dispatcher.setup(&mut world);

        Self {
            world,
            dispatcher,
            window,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let size = self
                .world
                .get_mut::<winit::dpi::PhysicalSize<u32>>()
                .unwrap();
            *size = new_size;
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}

struct RotateSystem;

impl<'a> specs::System<'a> for RotateSystem {
    type SystemData = (
        specs::WriteStorage<'a, Transform>,
        specs::ReadStorage<'a, Renderer>,
    );

    fn run(&mut self, (mut transforms, renderer): Self::SystemData) {
        for (transform, _) in (&mut transforms, &renderer).join() {
            transform.rotation = transform.rotation
                * cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.5))
                * cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_y(), cgmath::Deg(-0.5))
                * cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_x(), cgmath::Deg(0.5));
        }
    }
}
