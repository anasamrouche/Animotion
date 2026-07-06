use std::num::NonZero;

use crate::core::AnimRender;
use crate::core::camera::CameraUniform;
use crate::core::time::Time;
use crate::{
    core::{Vertex, objects::AnimObject, scene::Scene},
    gpu::context::GPUContext,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferDescriptor, BufferUsages, DepthStencilState,
    RenderPipeline, ShaderSource, ShaderStages, TextureFormat, VertexBufferLayout,
};

pub struct BufferPack {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub vertices_vec: Vec<Vertex>,
    pub indices_vec: Vec<u32>,
}

impl BufferPack {
    fn new(vertex_buffer: Buffer, index_buffer: Buffer) -> Self {
        Self {
            vertex_buffer,
            index_buffer,
            vertices_vec: Vec::new(),
            indices_vec: Vec::new(),
        }
    }
}

pub struct Renderer {
    pub curves_pipeline: RenderPipeline,
    pub surfaces_pipeline: RenderPipeline,
    pub curves_buffer: BufferPack,
    pub surfaces_buffer: BufferPack,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: Buffer,
    pub camera_bind_group: BindGroup,
    pub time_buffer: Buffer,
    pub time_bind_group: BindGroup,
}

impl Renderer {
    pub fn new(context: &GPUContext, fps: u32) -> Self {
        let curves_vertex_buffer = context.device.create_buffer(&BufferDescriptor {
            label: Some("Curves Vertex Buffer"),
            size: 2u64.pow(28),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let curves_index_buffer = context.device.create_buffer(&BufferDescriptor {
            label: Some("Curves Index Buffer"),
            size: 2u64.pow(26),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: ShaderSource::Wgsl(include_str!("./shader.wgsl").into()),
            });

        let surfaces_vertex_buffer = context.device.create_buffer(&BufferDescriptor {
            label: Some("Surfaces Vertex Buffer"),
            size: 2u64.pow(28),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let surfaces_index_buffer = context.device.create_buffer(&BufferDescriptor {
            label: Some("Surfaces Index Buffer"),
            size: 2u64.pow(26),
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_uniform = CameraUniform::new();
        let camera_buffer = context.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera buffer descriptor"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Camera bind groupe layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let time = Time::new(fps);
        let time_buffer = context.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Time Buffer"),
            contents: bytemuck::cast_slice(&[time]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let time_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: Some("Time Bind Group Layout"),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: NonZero::new(8),
                        },
                        count: None,
                    }],
                });

        let time_bind_group = context.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Time Bind Group"),
            layout: &time_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: time_buffer.as_entire_binding(),
            }],
        });

        let render_pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[
                        Some(&camera_bind_group_layout),
                        Some(&time_bind_group_layout),
                    ],
                    immediate_size: 0,
                });

        let curves_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[VertexBufferLayout {
                            array_stride: std::mem::size_of::<crate::core::Vertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: 0,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: std::mem::size_of::<[f32; 4]>() as u64,
                                },
                            ],
                        }],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: context.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),

                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::LineList,
                        strip_index_format: None,
                        cull_mode: Some(wgpu::Face::Front),
                        front_face: wgpu::FrontFace::Cw,
                        ..Default::default()
                    },

                    depth_stencil: Some(DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: Some(true),
                        depth_compare: Some(wgpu::CompareFunction::Less),
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                });

        let camera_bind_group = context.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: &camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let surfaces_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[VertexBufferLayout {
                            array_stride: std::mem::size_of::<crate::core::Vertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: 0,
                                },
                                wgpu::VertexAttribute {
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x4,
                                    offset: std::mem::size_of::<[f32; 4]>() as u64,
                                },
                            ],
                        }],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: context.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),

                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        ..Default::default()
                    },

                    depth_stencil: Some(DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: Some(true),
                        depth_compare: Some(wgpu::CompareFunction::Less),
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState::default(),
                    multiview_mask: None,
                    cache: None,
                });

        let curves_buffer = BufferPack::new(curves_vertex_buffer, curves_index_buffer);
        let surfaces_buffer = BufferPack::new(surfaces_vertex_buffer, surfaces_index_buffer);

        Self {
            curves_pipeline,
            surfaces_pipeline,
            curves_buffer,
            surfaces_buffer,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            time_buffer,
            time_bind_group,
        }
    }

    pub fn render(&mut self, context: &GPUContext, scene: &mut Scene) -> anyhow::Result<()> {
        let output = match context.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                context
                    .surface
                    .configure(&context.device, &context.surface_config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                context
                    .surface
                    .configure(&context.device, &context.surface_config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device");
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        self.curves_buffer.vertices_vec.clear();
        self.curves_buffer.indices_vec.clear();
        self.surfaces_buffer.vertices_vec.clear();
        self.surfaces_buffer.indices_vec.clear();

        for object in &scene.objects {
            match object {
                AnimObject::Line(line) => {
                    let vertex_count = self.surfaces_buffer.vertices_vec.len() as u32;

                    self.surfaces_buffer
                        .vertices_vec
                        .extend(line.get_vertices());
                    let prim_object_indices = line.get_indices();
                    let object_indices = prim_object_indices.iter().map(|i| i + vertex_count);
                    self.surfaces_buffer.indices_vec.extend(object_indices);
                }
                AnimObject::Sphere(sphere) => {
                    let vertex_count = self.surfaces_buffer.vertices_vec.len() as u32;

                    self.surfaces_buffer
                        .vertices_vec
                        .extend(sphere.get_vertices());
                    let prim_object_indices = sphere.get_indices();
                    let object_indices = prim_object_indices.iter().map(|i| i + vertex_count);
                    self.surfaces_buffer.indices_vec.extend(object_indices);
                }
                AnimObject::Tetrahedron(tetrahedron) => {
                    let vertex_count = self.surfaces_buffer.vertices_vec.len() as u32;

                    self.surfaces_buffer
                        .vertices_vec
                        .extend(tetrahedron.get_vertices());
                    let prim_object_indices = tetrahedron.get_indices();
                    let object_indices = prim_object_indices.iter().map(|i| i + vertex_count);
                    self.surfaces_buffer.indices_vec.extend(object_indices);
                }
                AnimObject::AnimFloat(_) => {}
            }
        }

        scene.camera_controller.update_camera(&mut scene.camera);
        self.camera_uniform.update_view(&scene.camera);

        context.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        context
            .queue
            .write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[scene.time]));
        context.queue.write_buffer(
            &self.curves_buffer.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.curves_buffer.vertices_vec),
        );
        context.queue.write_buffer(
            &self.curves_buffer.index_buffer,
            0,
            bytemuck::cast_slice(&self.curves_buffer.indices_vec),
        );
        context.queue.write_buffer(
            &self.surfaces_buffer.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.surfaces_buffer.vertices_vec),
        );
        context.queue.write_buffer(
            &self.surfaces_buffer.index_buffer,
            0,
            bytemuck::cast_slice(&self.surfaces_buffer.indices_vec),
        );

        {
            let background_color = scene.background_color_f64();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background_color[0],
                            g: background_color[1],
                            b: background_color[2],
                            a: background_color[3],
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &context.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &self.time_bind_group, &[]);
            render_pass.set_pipeline(&self.curves_pipeline);
            render_pass.set_vertex_buffer(0, self.curves_buffer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.curves_buffer.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..self.curves_buffer.indices_vec.len() as u32, 0, 0..1);

            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_pipeline(&self.surfaces_pipeline);
            render_pass.set_vertex_buffer(0, self.surfaces_buffer.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.surfaces_buffer.index_buffer.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..self.surfaces_buffer.indices_vec.len() as u32, 0, 0..1);
        }

        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
