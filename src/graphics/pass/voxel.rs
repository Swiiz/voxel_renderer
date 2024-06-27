use std::{mem::size_of, num::NonZeroU64};

use crate::{
    graphics::{
        camera::{Camera, CameraRenderParams},
        ctx::GraphicsCtx,
        wgsl::load_wgsl_with_preprocessor,
        Frame,
    },
    maths::{Vec2f, Vec2u},
};
use bytemuck::{Pod, Zeroable};
use util::{DeviceExt, StagingBelt};
use wgpu::*;

pub struct VoxelRenderingPass {
    pipeline: ComputePipeline,
    params: Buffer,
    camera_params: Buffer,
    bind_group: BindGroup,
    staging_belt: StagingBelt,
}

const PARAMS_SIZE: u64 = size_of::<VoxelPassParams>() as u64;
const CAMERA_PARAMS_SIZE: u64 = size_of::<CameraRenderParams>() as u64;

impl VoxelRenderingPass {
    pub fn new(ctx: &GraphicsCtx, output: TextureView) -> Self {
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(
                    load_wgsl_with_preprocessor("wgsl/voxel/main.wgsl").into(),
                ),
            });
        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });
        let pipeline = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: Some(&pipeline),
                module: &shader,
                entry_point: "main",
                compilation_options: PipelineCompilationOptions::default(),
            });

        let params = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: PARAMS_SIZE,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let params_binding = params.as_entire_binding();

        let camera_params = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: CAMERA_PARAMS_SIZE,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });
        let camera_params_binding = camera_params.as_entire_binding();

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&output),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: params_binding,
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: camera_params_binding,
                },
            ],
        });

        let staging_belt = StagingBelt::new(PARAMS_SIZE + CAMERA_PARAMS_SIZE); //TODO: improve this?

        Self {
            pipeline,
            params,
            camera_params,
            bind_group,
            staging_belt,
        }
    }

    pub fn run(&mut self, frame: &mut Frame, camera: &Camera, params: VoxelPassParams) {
        self.staging_belt
            .write_buffer(
                &mut frame.render.encoder,
                &self.params,
                0,
                NonZeroU64::new(PARAMS_SIZE).unwrap(),
                &frame.ctx.device,
            )
            .copy_from_slice(bytemuck::bytes_of(&params));

        self.staging_belt
            .write_buffer(
                &mut frame.render.encoder,
                &self.camera_params,
                0,
                NonZeroU64::new(CAMERA_PARAMS_SIZE).unwrap(),
                &frame.ctx.device,
            )
            .copy_from_slice(bytemuck::bytes_of(
                &camera.render_params(Vec2u::new(params.width, params.height)),
            ));

        self.staging_belt.finish();
        {
            let mut cpass = frame.render.encoder.begin_compute_pass(&Default::default());
            cpass.set_pipeline(&self.pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch_workgroups(params.width / 16, params.height / 16, 1);
        }
    }

    pub fn post_render(&mut self) {
        self.staging_belt.recall();
    }
}

const CONFIG_SIZE: u64 = size_of::<VoxelPassParams>() as u64;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct VoxelPassParams {
    pub width: u32,
    pub height: u32,
    pub time: f32,
}
