use camera::Camera;
use ctx::{GraphicsCtx, RenderCtx};
use pass::{
    postproc::PostProcessingPass,
    voxel::{VoxelPassParams, VoxelRenderingPass},
};
use wgpu::SurfaceTarget;

pub mod camera;
pub mod ctx;
pub mod pass;
pub mod wgsl;

pub struct Graphics<'w> {
    pub ctx: GraphicsCtx<'w>,
    voxel_pass: VoxelRenderingPass,
    postproc_pass: PostProcessingPass,
}

impl<'w> Graphics<'w> {
    pub fn new(window_size: impl Into<(u32, u32)>, target: impl Into<SurfaceTarget<'w>>) -> Self {
        let window_size = window_size.into();
        let ctx = GraphicsCtx::new(window_size, target);

        Self::new_from_ctx(ctx)
    }

    fn new_from_ctx(ctx: GraphicsCtx<'w>) -> Self {
        let (postproc_pass, post_proc_input) = PostProcessingPass::new(&ctx, ctx.window_size());
        let voxel_pass = VoxelRenderingPass::new(&ctx, post_proc_input);

        Self {
            voxel_pass,
            postproc_pass,
            ctx,
        }
    }

    pub fn refresh(&mut self) {
        let ctx = &self.ctx;
        let (postproc_pass, post_proc_input) = PostProcessingPass::new(&ctx, ctx.window_size());
        let voxel_pass = VoxelRenderingPass::new(&ctx, post_proc_input);

        self.voxel_pass = voxel_pass;
        self.postproc_pass = postproc_pass;
    }

    pub fn resize(&mut self, window_size: impl Into<(u32, u32)>) {
        self.ctx.resize(window_size.into());
    }

    pub fn render(&mut self, camera: &Camera, time: f32) {
        if let Some(mut frame) = self.ctx.next_frame() {
            let (width, height) = frame.ctx.window_size();

            self.voxel_pass.run(
                &mut frame,
                camera,
                VoxelPassParams {
                    time,
                    width,
                    height,
                },
            );
            self.postproc_pass.run(&mut frame);

            frame.present();

            self.voxel_pass.post_render();
        }
    }
}

pub struct Frame<'a> {
    pub ctx: &'a GraphicsCtx<'a>,
    pub render: RenderCtx,
}
