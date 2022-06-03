use winit::{event::*, window::Window};

use wgpu::util::DeviceExt;

use crate::misc::*;

use cgmath::prelude::*;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        color: [0.0, 0.0, 1.0],
    }, // A
    Vertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    }, // B
    Vertex {
        position: [1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // C
    Vertex {
        position: [-1.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // D
];

pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    // diffuse_bind_group: wgpu::BindGroup,
    // diffuse_texture: texture::Texture,
    // camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    // camera_controller: CameraController,
    // instances: Vec<Instance>,
    // instance_buffer: wgpu::Buffer,
    sound_bind_group: wgpu::BindGroup,
    i: f32,
    sound_texture: wgpu::Texture,
    texture_size: wgpu::Extent3d,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    sound_texture_view: wgpu::TextureView,
    sound_sampler: wgpu::Sampler,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        // let camera_controller = CameraController::new(0.2);

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
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
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // let diffuse_bytes = include_bytes!("happy-tree.png"); // CHANGED!
        // let diffuse_texture =
        //     texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        // let texture_bind_group_layout =
        //     device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         entries: &[
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 0,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 ty: wgpu::BindingType::Texture {
        //                     multisampled: false,
        //                     view_dimension: wgpu::TextureViewDimension::D2,
        //                     sample_type: wgpu::TextureSampleType::Float { filterable: true },
        //                 },
        //                 count: None,
        //             },
        //             wgpu::BindGroupLayoutEntry {
        //                 binding: 1,
        //                 visibility: wgpu::ShaderStages::FRAGMENT,
        //                 // This should match the filterable field of the
        //                 // corresponding Texture entry above.
        //                 ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        //                 count: None,
        //             },
        //         ],
        //         label: Some("texture_bind_group_layout"),
        //     });

        // let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     layout: &texture_bind_group_layout,
        //     entries: &[
        //         wgpu::BindGroupEntry {
        //             binding: 0,
        //             resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
        //         },
        //         wgpu::BindGroupEntry {
        //             binding: 1,
        //             resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
        //         },
        //     ],
        //     label: Some("diffuse_bind_group"),
        // });

        let texture_size = wgpu::Extent3d {
            width: 4,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture_data = [
            10, 10, 10, 255, 30, 30, 30, 255, 50, 50, 50, 255, 70, 70, 70, 255,
        ];
        let sound_texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("sound_texture"),
        });

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &sound_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &texture_data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * 16),
                rows_per_image: std::num::NonZeroU32::new(1),
            },
            texture_size,
        );

        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let sound_texture_view = sound_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sound_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

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

        let sound_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sound_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sound_sampler),
                },
            ],
            label: Some("sound_bind_group"),
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let num_vertices = VERTICES.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        // let camera = Camera {
        //     // position the camera one unit up and 2 units back
        //     // +z is out of the screen
        //     eye: (0.0, 0.0, 2.0).into(),
        //     // have it look at the origin
        //     target: (0.0, 0.0, 0.0).into(),
        //     // which way is "up"
        //     up: cgmath::Vector3::unit_y(),
        //     aspect: config.width as f32 / config.height as f32,
        //     fovy: 45.0,
        //     znear: 0.1,
        //     zfar: 100.0,
        // };

        let mut camera_uniform = CameraUniform::new(size.width as f32, size.height as f32);
        // camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // println!("{:?}", camera_buffer);

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",                                // 1.
                buffers: &[Vertex::desc() /* , InstanceRaw::desc()*/], // 2.
            },
            fragment: Some(wgpu::FragmentState {
                // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
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

        let i = 0.0;

        // let render_pipeline_layout =
        //     device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //         label: Some("Render Pipeline Layout"),
        //         bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
        //         push_constant_ranges: &[],
        //     });

        // let instances = (0..NUM_INSTANCES_PER_ROW)
        //     .flat_map(|z| {
        //         (0..NUM_INSTANCES_PER_ROW).map(move |x| {
        //             let position = cgmath::Vector3 {
        //                 x: x as f32,
        //                 y: 0.0,
        //                 z: z as f32,
        //             } - INSTANCE_DISPLACEMENT;

        //             let rotation = if position.is_zero() {
        //                 // this is needed so an object at (0, 0, 0) won't get scaled to zero
        //                 // as Quaternions can effect scale if they're not created correctly
        //                 cgmath::Quaternion::from_axis_angle(
        //                     cgmath::Vector3::unit_z(),
        //                     cgmath::Deg(0.0),
        //                 )
        //             } else {
        //                 cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
        //             };

        //             Instance { position, rotation }
        //         })
        //     })
        //     .collect::<Vec<_>>();

        // let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        // let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: Some("Instance Buffer"),
        //     contents: bytemuck::cast_slice(&instance_data),
        //     usage: wgpu::BufferUsages::VERTEX,
        // });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            // diffuse_bind_group,
            // diffuse_texture,
            // camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            // camera_controller,
            // instances,
            // instance_buffer,
            sound_bind_group,
            i,
            sound_texture,
            texture_size,
            texture_bind_group_layout,
            sound_texture_view,
            sound_sampler,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        // self.camera_controller.update_camera(&mut self.camera);
        // self.camera_uniform.update_view_proj(&self.camera);
        // self.queue.write_buffer(
        //     &self.camera_buffer,
        //     0,
        //     bytemuck::cast_slice(&[self.camera_uniform]),
        // );

        let texture_data = [
            (10.0 * f32::cos(self.i)) as u8,
            (10.0 * f32::sin(self.i)) as u8,
            (10.0 * f32::tan(self.i)) as u8,
            255,
            (30.0 * f32::cos(self.i)) as u8,
            (30.0 * f32::sin(self.i)) as u8,
            (30.0 * f32::tan(self.i)) as u8,
            255,
            (50.0 * f32::cos(self.i)) as u8,
            (50.0 * f32::sin(self.i)) as u8,
            (50.0 * f32::tan(self.i)) as u8,
            255,
            (70.0 * f32::cos(self.i)) as u8,
            (70.0 * f32::sin(self.i)) as u8,
            (70.0 * f32::tan(self.i)) as u8,
            255,
        ];

        println!("{}", texture_data[1]);

        self.queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &self.sound_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &texture_data,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * 16),
                rows_per_image: std::num::NonZeroU32::new(1),
            },
            self.texture_size,
        );

        self.sound_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.sound_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sound_sampler),
                },
            ],
            label: Some("sound_bind_group"),
        });
        self.i = self.i + 0.01;
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what [[location(0)]] in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline); // 2.
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.sound_bind_group, &[]); // NEW!
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.

            // render_pass.draw(0..self.num_vertices, 0..1); // 3.
            render_pass.draw_indexed(
                0..self.num_indices,
                0,
                /*0..self.instances.len() as _*/ 0..1,
            );
            // 2.
        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
