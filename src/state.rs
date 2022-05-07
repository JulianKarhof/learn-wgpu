use cgmath::num_traits::ToPrimitive;
use cgmath::vec2;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::{event::WindowEvent, window::Window};

use crate::camera::{Camera, CameraUniform};
use crate::circle::CirclePipeline;
use crate::rect::RectPipeline;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,

    clear_color: wgpu::Color,

    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    rect_pipeline: RectPipeline,
    circle_pipeline: CirclePipeline,

    last_cursor_position: PhysicalPosition<f64>,
    mouse_pressed: bool,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

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
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
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

        let clear_color = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };

        let camera = Camera {
            height: config.height as f32,
            width: config.width as f32,
            offset: vec2(0.0, 0.0),
            zoom: 1.0,
            mouse_pos: vec2(0.0, 0.0),
            limits: [0.0, config.width as f32, config.height as f32, 0.0],
        };

        let mut camera_uniform = CameraUniform::new();
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

        let rect_pipeline = RectPipeline::new(&device, &camera_bind_group_layout, &config);
        let circle_pipeline = CirclePipeline::new(&device, &camera_bind_group_layout, &config);

        let last_cursor_position = PhysicalPosition::new(0.0, 0.0);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            clear_color,
            camera,
            camera_bind_group,
            camera_buffer,
            camera_uniform,
            rect_pipeline,
            circle_pipeline,
            last_cursor_position,
            mouse_pressed: false,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera.height = new_size.height as f32;
            self.camera.width = new_size.width as f32;

            self.camera_uniform.update_view_proj(&mut self.camera);

            self.queue.write_buffer(
                &self.camera_buffer,
                0,
                bytemuck::cast_slice(&[self.camera_uniform]),
            );
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::PixelDelta(PhysicalPosition { x: _, y }) => {
                        let left0 = self.camera.limits[0];
                        let right0 = self.camera.limits[1];
                        let bottom0 = self.camera.limits[2];
                        let top0 = self.camera.limits[3];
                        let w0 = self.camera_uniform.get_absolute_mouse_pos(&self.camera);
                        let zoom_factor0 = self.camera.zoom;

                        self.camera.zoom -= y.to_f32().unwrap() * 0.002;
                        self.camera_uniform.update_view_proj(&mut self.camera);

                        let zoom_factor_ratio = self.camera.zoom / zoom_factor0;

                        let left = w0.x - (w0.x - left0) * zoom_factor_ratio;
                        let right = w0.x - (w0.x - right0) * zoom_factor_ratio;
                        let bottom = w0.y - (w0.y - bottom0) * zoom_factor_ratio;
                        let top = w0.y - (w0.y - top0) * zoom_factor_ratio;

                        self.camera.limits = [left, right, bottom, top];

                        self.camera_uniform.update_view_proj(&mut self.camera);
                        self.queue.write_buffer(
                            &self.camera_buffer,
                            0,
                            bytemuck::cast_slice(&[self.camera_uniform]),
                        );
                    }
                    _ => {}
                };

                return true;
            }

            WindowEvent::MouseInput { button, state, .. } => {
                if *button == MouseButton::Left {
                    if *state == ElementState::Released {
                        self.mouse_pressed = false;
                    }

                    if *state == ElementState::Pressed {
                        self.mouse_pressed = true;
                    }
                }

                return true;
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.camera.mouse_pos =
                    vec2(position.x.to_f32().unwrap(), position.y.to_f32().unwrap());

                if self.mouse_pressed {
                    let difference: PhysicalPosition<f32> = PhysicalPosition {
                        x: self.last_cursor_position.x.to_f32().unwrap()
                            - position.x.to_f32().unwrap(),
                        y: self.last_cursor_position.y.to_f32().unwrap()
                            - position.y.to_f32().unwrap(),
                    };

                    self.camera.limits[0] += difference.x * self.camera.zoom;
                    self.camera.limits[1] += difference.x * self.camera.zoom;
                    self.camera.limits[2] += difference.y * self.camera.zoom;
                    self.camera.limits[3] += difference.y * self.camera.zoom;

                    self.camera_uniform.update_view_proj(&mut self.camera);
                    self.queue.write_buffer(
                        &self.camera_buffer,
                        0,
                        bytemuck::cast_slice(&[self.camera_uniform]),
                    );
                }

                self.last_cursor_position = *position;

                return true;
            }
            _ => {
                self.clear_color = wgpu::Color {
                    r: 0.01,
                    g: 0.01,
                    b: 0.1,
                    a: 1.0,
                };

                return false;
            }
        }
    }

    pub fn update(&mut self) {}

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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.clear_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        self.rect_pipeline
            .render(&mut render_pass, &self.camera_bind_group);

        self.circle_pipeline
            .render(&mut render_pass, &self.camera_bind_group);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
