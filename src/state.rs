use log::info;
use std::default::Default;
use std::time::Instant;
use log::log;
use wgpu::MemoryHints;
use wgpu::naga::SwitchValue;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::keyboard::{Key, KeyCode};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;
use crate::camera;
use crate::camera::CamValues;

pub(crate) struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: &'a Window,
    start_time: Instant,
    time_buffer: wgpu::Buffer,
    time_bind_group: wgpu::BindGroup,
    cam_values: CamValues,
    cam_buffer: wgpu::Buffer,
    cam_bind_group: wgpu::BindGroup,
}

impl<'a> State<'a> {
    // Creating some of the wgpu types requires async code
    pub(crate) async fn new(window: &'a Window) -> State<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        let size = window.inner_size();
        let surface = instance.create_surface(window).unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
                memory_hints: MemoryHints::default(),
            },
            None, // Trace path
        ).await.unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });








        let start_time = std::time::Instant::now();
        let time_data = 0.0f32.to_ne_bytes();
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Time Buffer"),
            contents: &time_data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let time_bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(4),
                },
                count: None,
            }
            ],
            label: Some("time_bind_group_layout"),
        });
        let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &time_bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
            label: Some("time_bind_group"),
        });




        let start_cam = camera::CamValues::default();

        let cam_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Cam buffer"),
            contents: bytemuck::cast_slice(&[start_cam]),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        });
        let cam_bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(16), // 3x f32 -> 3x4 bytes but because of weird shit with alignment it is 16bytes
                },
                count: None,
            }
            ],
            label: Some("cam_bind_group_layout"),
        });
        let cam_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &cam_bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: cam_buffer.as_entire_binding(),
            }],
            label: Some("cam_bind_group"),
        });





        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &time_bind_layout,
                    &cam_bind_layout
                ],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
            cache: None, // 6.
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            start_time,
            time_buffer,
            time_bind_group,
            cam_values: start_cam,
            cam_buffer,
            cam_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if(new_size.width == 0 || new_size.height == 0){
            return;
        }
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        let ratio = self.config.width as f32 / self.config.height as f32;
        self.cam_values.set_ratio(ratio);
        self.queue.write_buffer(&self.cam_buffer, 0, bytemuck::cast_slice(&[self.cam_values]));
        //info!("Resizing to {:?}|{:?}", self.config.width, self.config.height);
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        /*match event {
            WindowEvent::KeyboardInput {
                device_id, event, is_synthetic
            } => {
                if let Key::Character(c) = &event.logical_key {
                    let motion = match c.as_str(){
                        "w" => {[0.0f32, 1.0]},
                        "a" => {[-1.0, 0.0]},
                        "s" => {[0.0, -1.0]},
                        "d" => {[1.0, 0.0]},
                        _ => {
                            return false;
                        }
                    };

                    println!("{:?}", self.cam_values);
                    self.cam_values.offset(motion);
                    self.queue.write_buffer(&self.cam_buffer, 0, bytemuck::cast_slice(&[self.cam_values]));

                    return false;
                }
                return false;
            },
            _ => false,
        }*/
        false
    }

    pub(crate) fn update(&mut self, input_helper: &WinitInputHelper) {
        let mut motion = [0.0f32, 0.0];
        if input_helper.key_held(KeyCode::KeyA) {
            motion[0] = motion[0] - 1.0 / self.cam_values.get_zoom().exp();
        }
        if input_helper.key_held(KeyCode::KeyD) {
            motion[0] = motion[0] + 1.0 / self.cam_values.get_zoom().exp();
        }
        if input_helper.key_held(KeyCode::KeyW) {
            motion[1] = motion[1] + 1.0 / self.cam_values.get_zoom().exp();
        }
        if input_helper.key_held(KeyCode::KeyS) {
            motion[1] = motion[1] - 1.0 / self.cam_values.get_zoom().exp();
        }
        if input_helper.key_pressed(KeyCode::Space) {
            self.cam_values.set_offset([0.0,0.0]);
        }
        self.cam_values.offset(motion);


        self.cam_values.zoom(input_helper.scroll_diff().1);
        //println!("{:?}", self.cam_values);
        self.queue.write_buffer(&self.cam_buffer, 0, bytemuck::cast_slice(&[self.cam_values]));

    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let out = self.surface.get_current_texture()?;

        let elapsed = self.start_time.elapsed().as_secs_f32();
        self.queue.write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[elapsed]));



        let view = out.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        // This is what @location(0) in the fragment shader targets
                        Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations::default(),
                        })
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                // NEW!
                render_pass.set_pipeline(&self.render_pipeline); // 2.
                render_pass.set_bind_group(0, &self.time_bind_group, &[]);
                render_pass.set_bind_group(1, &self.cam_bind_group, &[]);
                render_pass.draw(0..6, 0..1);
            }
            // submit will accept anything that implements IntoIter
            self.queue.submit(std::iter::once(encoder.finish()));
            out.present();

            Ok(())
        }
    }

}
