use std::collections::HashMap;

use bytemuck;
use sdl2::keyboard::Keycode;
use wgpu::util::DeviceExt;

use crate::node;
use crate::node_collection;

use crate::graphics;
use graphics::camera;
use graphics::instances::instance_collection::InstanceCollection;
use graphics::instances::{instance, instance_collection};
use graphics::models::{material, model, model_collection};
use graphics::texture;
use graphics::vertex::Vertex;

use cgmath::prelude::*;

pub struct State {
    window: sdl2::video::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: (u32, u32),
    render_pipeline: wgpu::RenderPipeline,
    fallback_material: material::Material,
    default_material: Option<material::Material>,
    use_default_material: bool,
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    models: model_collection::ModelCollection,
    depth_texture: texture::Texture,
    instance_collections: Vec<instance_collection::InstanceCollection>,
    node_collection: node_collection::NodeCollection,
    node_to_instance_lookup: HashMap<node::NodeId, instance::Instance>,
}

impl super::Scene for State {
    fn new(context: &sdl2::Sdl, default_texture_path: Option<String>) -> Self {
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN | wgpu::Backends::METAL,
            dx12_shader_compiler: Default::default(),
        });

        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .metal_view()
            .resizable()
            .build()
            .unwrap();
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        // Adapter - translation layer between OS native graphics API and wgpu
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let size = window.size();

        // device - Logical device requested from adapter, to look
        // like we are the only thing using the GPU
        // (Since multiple apps can use the adapter?)
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
                label: None,
            },
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let default_material: Option<material::Material> = match default_texture_path {
            Some(path) => {
                let material = pollster::block_on(material::Material::load(
                    "default_material".to_string(),
                    &path,
                    &device,
                    &queue,
                ));
                match material {
                    Ok(mat) => Some(mat),
                    Err(error) => {
                        println!(
                            "Error loading material from file {} - {}",
                            &path, error.message
                        );
                        None
                    }
                }
            }
            None => None,
        };

        let fallback_material = pollster::block_on(material::Material::load(
            "fallback_material".to_string(),
            "fallback-texture.jpg",
            &device,
            &queue,
        ))
        .expect("Could not load fallback texture");

        let camera = camera::Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = camera::CameraController::new(0.2);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture::Texture::create_texture_bind_group_layout(&device),
                    &camera_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::ModelVertex::desc(), instance::InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut models = model_collection::ModelCollection::new();
        let cube_descriptor = model::LoadModelDescriptor::new("cube.obj", &device, &queue);
        let load_model = |id| {
            let model = pollster::block_on(model::load_model(cube_descriptor, id));
            let model = model.unwrap();
            return model;
        };
        let cube_id = models.add(load_model);
        let cube_instance_collection = instance_collection::InstanceCollection::new(cube_id);

        let instance_collections = vec![cube_instance_collection];
        let node_collection = node_collection::NodeCollection::new();
        let node_to_instance_lookup: HashMap<node::NodeId, instance::Instance> = HashMap::new();

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            fallback_material,
            default_material,
            use_default_material: false,
            camera,
            camera_controller,
            camera_buffer,
            camera_uniform,
            camera_bind_group,
            models,
            depth_texture,
            instance_collections,
            node_collection,
            node_to_instance_lookup,
        }
    }

    fn resize(&mut self, new_size: (u32, u32)) {
        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0;
            self.config.height = new_size.1;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    fn input(&mut self, event: &sdl2::event::Event) -> bool {
        if self.camera_controller.process_events(event) {
            return true;
        }
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::D => {
                    self.use_default_material = !self.use_default_material;
                    true
                }
                _ => false,
            },
            _ => false,
        };
        false
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn add_node_to_scene(&mut self, node: node::Node) {
        // TODO Need to determine _what_ collection here
        let instance_collection: &mut InstanceCollection = &mut self.instance_collections[0];
        let node_collection = &mut self.node_collection;
        if node_collection.iter().any(|n| n.id == node.id) {
            println!(
                "Node with ID {} has already been added to the scene",
                node.id
            );
            return;
        }

        let new_instance = instance::Instance {
            position: cgmath::Vector3::new(
                node.position.x as f32,
                0 as f32,
                node.position.y as f32,
            ),
            rotation: cgmath::Quaternion::zero(),
        };

        self.node_to_instance_lookup.insert(node.id, new_instance);
        node_collection.add(node);
        instance_collection.add(new_instance);
    }

    fn remove_node_from_scene(&mut self, id: node::NodeId) {
        let instance_collection: &mut InstanceCollection = &mut self.instance_collections[0];
        let node_collection = &mut self.node_collection;

        if !node_collection.iter().any(|n| n.id == id) {
            println!("No node with ID {} has been added to the scene", id);
            return;
        }

        node_collection.remove(id);
        let instance = self.node_to_instance_lookup.get(&id);
        instance_collection.remove(*instance.unwrap());
        self.node_to_instance_lookup.remove(&id);
    }

    fn render(&mut self, clear_colour: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let instance_data = instance_collection::InstanceCollection::get_instance_render_data(
            &self.instance_collections,
        );
        let instance_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data.data),
                usage: wgpu::BufferUsages::VERTEX,
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_colour),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

            use model::DrawModel;
            for collection in self.instance_collections.iter() {
                let id = collection.model;
                let model = self.models.find(id);
                let model = model.unwrap();
                let range = &instance_data
                    .indexes
                    .iter()
                    .find(|value| value.0 == id)
                    .unwrap()
                    .1;
                let mesh = &model.meshes[0];

                let material =
                    if self.use_default_material == false && model.materials.is_empty() == false {
                        &model.materials[0]
                    } else {
                        &self
                            .default_material
                            .as_ref()
                            .unwrap_or(&self.fallback_material)
                    };

                render_pass.draw_mesh_instanced(
                    mesh,
                    material,
                    range.clone(),
                    &self.camera_bind_group,
                );
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
