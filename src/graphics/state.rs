use bytemuck;
use wgpu::util::DeviceExt;
use wgpu::BindGroupLayout;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};
use winit::{event::WindowEvent, window::Window};

use super::camera;
use super::instances::{instance, instance_collection};
use super::models::{model, model_collection};
use super::texture;
use super::vertex::Vertex;

use cgmath::prelude::*;

pub struct State {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: Window,
    pub render_pipeline: wgpu::RenderPipeline,
    diffuse_bind_groups: [wgpu::BindGroup; 1],
    diffuse_textures: [texture::Texture; 1],
    pub texture_index: usize,
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    pub move_offset: f32,
    models: model_collection::ModelCollection,
    depth_texture: texture::Texture,
    texture_bind_group_layout: BindGroupLayout,
    instance_collections: Vec<instance_collection::InstanceCollection>,
}

const NUM_INSTANCES_PER_ROW: u32 = 10;

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        // Adapter - translation layer between OS native graphics API and wgpu
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // device - Logical device requested from adapter, to look
        // like we are the only thing using the GPU
        // (Since multiple apps can use the adapter?)
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_defaults(),
                    label: None,
                },
                None,
            )
            .await
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
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let diffuse_bytes_sarah = include_bytes!("../../data/sarah.jpg");
        let diffuse_texture_sarah =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes_sarah, "sarah").unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group_sarah = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_sarah.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture_sarah.sampler),
                },
            ],
            label: Some("diffuse_bind_group_sarah"),
        });

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
            source: wgpu::ShaderSource::Wgsl(include_str!("../shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
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
        let move_offset = 0.0;

        const SPACE_BETWEEN: f32 = 3.0;

        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
        let cube_positions = vec![
            cgmath::Vector3 {
                x: 1.0,
                y: 0.0,
                z: 1.0,
            },
            cgmath::Vector3 {
                x: 4.0,
                y: 0.0,
                z: 1.0,
            },
            cgmath::Vector3 {
                x: 7.0,
                y: 0.0,
                z: 1.0,
            },
        ];
        let cuboid_positions = vec![
            cgmath::Vector3 {
                x: 1.0,
                y: 0.0,
                z: 4.0,
            },
            cgmath::Vector3 {
                x: 4.0,
                y: 0.0,
                z: 4.0,
            },
            cgmath::Vector3 {
                x: 7.0,
                y: 0.0,
                z: 4.0,
            },
        ];

        let mut models = model_collection::ModelCollection::new();
        let cube_descriptor = model::LoadModelDescriptor::new(
            "cube.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let load_model = |id| {
            let model = pollster::block_on(model::load_model(cube_descriptor, id));
            let model = model.unwrap();
            return model;
        };
        let cube_id = models.add(load_model);

        let cube_instances: Vec<instance::Instance> = cube_positions
            .into_iter()
            .map(move |position| instance::Instance { position, rotation })
            .collect();

        let mut cube_instance_collection = instance_collection::InstanceCollection::new(cube_id);
        cube_instances
            .into_iter()
            .for_each(|instance| cube_instance_collection.add(instance));

        let cuboid_descriptor = model::LoadModelDescriptor::new(
            "cuboid.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        );
        let load_model = |id| {
            let model = pollster::block_on(model::load_model(cuboid_descriptor, id));
            let model = model.unwrap();
            return model;
        };
        let cuboid_id = models.add(load_model);

        let cuboid_instances: Vec<instance::Instance> = cuboid_positions
            .into_iter()
            .map(move |position| instance::Instance { position, rotation })
            .collect();

        let mut cuboid_instance_collection =
            instance_collection::InstanceCollection::new(cuboid_id);
        cuboid_instances
            .into_iter()
            .for_each(|instance| cuboid_instance_collection.add(instance));

        let instance_collections = vec![cube_instance_collection, cuboid_instance_collection];

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            diffuse_bind_groups: [diffuse_bind_group_sarah],
            diffuse_textures: [diffuse_texture_sarah],
            texture_index: 0,
            camera,
            camera_controller,
            camera_buffer,
            camera_uniform,
            camera_bind_group,
            move_offset,
            models,
            depth_texture,
            texture_bind_group_layout,
            instance_collections,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event);
        match event {
            WindowEvent::CursorMoved { .. } => {}
            _ => (),
        }
        false
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        // if self.move_offset != 0.0 {
        //     self.update_instances()
        // }
    }

    // fn update_instances(&mut self) {
    //     for instance in self.instances.iter_mut() {
    //         instance.position.z += self.move_offset as f32;
    //     }

    //     let instance_data = self
    //         .instances
    //         .iter()
    //         .map(instance::Instance::to_raw)
    //         .collect::<Vec<_>>();
    //     self.instance_buffer = self
    //         .device
    //         .create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //             label: Some("Instance Buffer"),
    //             contents: bytemuck::cast_slice(&instance_data),
    //             usage: wgpu::BufferUsages::VERTEX,
    //         });
    //     self.move_offset = 0.0;
    // }

    pub fn render(&mut self, clear_colour: wgpu::Color) -> Result<(), wgpu::SurfaceError> {
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
            render_pass.set_bind_group(0, &self.diffuse_bind_groups[self.texture_index], &[]);
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
                render_pass.draw_mesh_instanced(&model.meshes[0], range.clone());
            }
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
