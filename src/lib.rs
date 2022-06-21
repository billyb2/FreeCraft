mod texture;
mod camera;

use camera::{Camera, CameraUniform};
use wgpu::{util::DeviceExt, Buffer, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

struct AppState {
    vertex_buffer: [Vertex; 24],
    indices: [u16; 36],
    bg_color: (f64, f64, f64),
    camera: Camera,
    moving_forward: bool,
    moving_backward: bool,
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
    //block_pos: Vec3,

}

impl AppState {
    fn new() -> Self {
        Self {
            bg_color: (0.0, 1.0, 1.0),
            vertex_buffer: [
                // Top face
                Vertex { position: [-1.0, -1.0, 1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [1.0, -1.0, 1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0, 1.0, 1.0], tex_coords: [0.0, 1.0] },
                // Bottom face
                Vertex { position: [-1.0, 1.0, -1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [1.0, 1.0, -1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [1.0, -1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
                // Right face
                Vertex { position: [1.0, -1.0, -1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [1.0, 1.0, -1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [1.0, 1.0, 1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [1.0, -1.0, 1.0], tex_coords: [0.0, 1.0] },
                // Left face
                Vertex { position: [-1.0, -1.0, 1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [-1.0, 1.0, 1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [-1.0, 1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
                // Front face
                Vertex { position: [1.0, 1.0, -1.0], tex_coords: [0.0, 0.0], }, // Top left
                Vertex { position: [-1.0, 1.0, -1.0], tex_coords: [0.0, 1.0], }, // Bottom left 
                Vertex { position: [-1.0, 1.0, 1.0], tex_coords: [1.0, 1.0], }, // Bottom right 
                Vertex { position: [1.0, 1.0, 1.0], tex_coords: [1.0, 0.0], }, // Top right 
                // Back face
                Vertex { position: [1.0, -1.0, 1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [-1.0, -1.0, 1.0], tex_coords: [1.0, 0.0] },
                Vertex { position: [-1.0, -1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [1.0, -1.0, -1.0], tex_coords: [0.0, 1.0] },
            ],
            indices: [
                0, 1, 2, 2, 3, 0, // top
                4, 5, 6, 6, 7, 4, // bottom
                8, 9, 10, 10, 11, 8, // right
                12, 13, 14, 14, 15, 12, // left
                16, 17, 18, 18, 19, 16, // front
                20, 21, 22, 22, 23, 20, // back           
            ],
            moving_forward: false,
            moving_backward: false,
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
            camera: Camera::new(),

        }
    }

    fn update(&mut self) {
        const SPEED: f32 = 0.1;

        if self.moving_forward {
            self.camera.move_forward(SPEED);

        }
       
        if self.moving_backward {
            self.camera.move_backward(SPEED);

        }

        if self.moving_left {
            self.camera.move_left(SPEED);

        }
        
        if self.moving_right {
            self.camera.move_right(SPEED);

        }

        if self.moving_up {
            self.camera.move_up(SPEED);

        }
         
        if self.moving_down {
            self.camera.move_down(SPEED);

        }

    }

    fn handle_input(&mut self, input: &KeyboardInput) {
        let keycode = match input.virtual_keycode {
            Some(keycode) => keycode,
            None => return,

        };

        let is_pressed = input.state == ElementState::Pressed;
        
        match keycode {
            VirtualKeyCode::W | VirtualKeyCode::Up => self.moving_forward = is_pressed,
            VirtualKeyCode::S | VirtualKeyCode::Down => self.moving_backward = is_pressed,
            VirtualKeyCode::A | VirtualKeyCode::Left => self.moving_left = is_pressed,
            VirtualKeyCode::D | VirtualKeyCode::Right => self.moving_right = is_pressed,
            VirtualKeyCode::Space => self.moving_up = is_pressed,
            VirtualKeyCode::LShift => self.moving_down = is_pressed,
            _ => (),

        };

    }

}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}

struct RendererState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
    render_pipeline_lines: wgpu::RenderPipeline,
    fill_mode: bool,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,

    camera_uniform: CameraUniform,
    camera_bind_group: wgpu::BindGroup,
    camera_buffer: Buffer,
}

impl RendererState {
    // Creating some of the wgpu types requires async code
    async fn new(window: &Window, app_state: &AppState) -> Self {
        let size = window.inner_size();

        #[cfg(not(target_arch = "wasm32"))]
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

        #[cfg(target_arch = "wasm32")]
        let instance = wgpu::Instance::new(wgpu::Backends::BROWSER_WEBGPU);

        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                //features: wgpu::Features::empty(),
                features: wgpu::Features::POLYGON_MODE_LINE,
                #[cfg(not(target_arch = "wasm32"))]
                limits: wgpu::Limits::default(),
                #[cfg(target_arch = "wasm32")]
                limits: wgpu::Limits::downlevel_webgl2_defaults(),
                label: None,

            }, 
            None
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let diffuse_bytes = include_bytes!("../assets/dirt.png");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "dirt.png").unwrap();
  
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

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&app_state.camera);

        let camera_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,

            }

        );


        let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                }
            ],
            label: Some("camera_bind_group"),
        });
             

 
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())

        });
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline_targets = [wgpu::ColorTargetState { // 4.
            format: config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[Vertex::desc()], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &render_pipeline_targets,
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None,
            
        };
        
        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        // THe exact same as the normal render pipliene, but render lines instead of filling
        let mut render_pipeline_descriptor_lines = render_pipeline_descriptor.clone();
        render_pipeline_descriptor_lines.primitive.polygon_mode = wgpu::PolygonMode::Line;
    
        let render_pipeline_lines = device.create_render_pipeline(&render_pipeline_descriptor_lines);
        
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&app_state.vertex_buffer),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                
            }
            
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index buffer"),
                contents: bytemuck::cast_slice(&app_state.indices),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                
            }
            
        );

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            render_pipeline_lines,
            vertex_buffer,
            index_buffer,
            diffuse_bind_group,
            diffuse_texture,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            // Use fill by default
            fill_mode: true,
        }

    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update_camera(&mut self, camera: &mut Camera) {
        camera.update_aspect(&self.config);
        self.camera_uniform.update_view_proj(camera);

    }

    fn render(&mut self, app_state: &AppState) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),

        });

        // Need to drop render_pass to output to screen
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what [[location(0)]] in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.2,
                                    b: 0.3,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });


            render_pass.set_pipeline(match self.fill_mode {
                true => &self.render_pipeline,
                false => &self.render_pipeline_lines,

            });
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..(app_state.indices.len().try_into().unwrap()), 0, 0..1);
            
        }

        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&app_state.vertex_buffer));
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(self.camera_uniform.view_proj()));
        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();

        Ok(())
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(800, 600));
        
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("canvas_div")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    };

    let mut app_state = AppState::new();
    let mut state = RendererState::new(&window, &app_state).await;

    app_state.camera.update_aspect(&state.config);

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            app_state.update();
            state.update_camera(&mut app_state.camera);
            state.render(&app_state).unwrap();

        },
        Event::MainEventsCleared => {
            // Trigger another redraw
            window.request_redraw();

        },
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit, 
            WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
                if input.virtual_keycode == Some(VirtualKeyCode::LAlt) {
                    state.fill_mode = input.state != ElementState::Pressed;
                    

                }

                app_state.handle_input(input);

            },
            _ => (),

        },
        _ => {}
    });
}

 
