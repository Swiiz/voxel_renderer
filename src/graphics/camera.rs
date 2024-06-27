use std::f32::NAN;

use bytemuck::{Pod, Zeroable};
use cgmath::InnerSpace;

use crate::maths::{Vec2u, Vec3f};

pub struct Camera {
    pub focal_length: f32,
    pub position: Vec3f,
    pub direction: Vec3f,
    pub up: Vec3f,
    pub controller: CameraController,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            focal_length: 1.0,
            position: Vec3f::new(0.0, 0.0, 0.0),
            direction: Vec3f::new(0.0, 0.0, -1.0),
            up: Vec3f::new(0.0, 1.0, 0.0),
            controller: CameraController {
                forward: false,
                backward: false,
                left: false,
                right: false,
                up: false,
                down: false,
                speed: 2.0,
            },
        }
    }
}

impl Camera {
    pub fn update_movement(&mut self, dt: f32) {
        if self.controller.forward {
            self.position += self.direction * self.controller.speed * dt;
        }
        if self.controller.backward {
            self.position -= self.direction * self.controller.speed * dt;
        }
        if self.controller.left {
            self.position += self.up.cross(self.direction) * self.controller.speed * dt;
        }
        if self.controller.right {
            self.position -= self.up.cross(self.direction) * self.controller.speed * dt;
        }
        if self.controller.up {
            self.position += self.up * self.controller.speed * dt;
        }
        if self.controller.down {
            self.position -= self.up * self.controller.speed * dt;
        }
    }

    pub fn render_params(&self, dims: Vec2u) -> CameraRenderParams {
        let aspect_ratio = dims.x as f32 / dims.y as f32;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let w = -self.direction.normalize();
        let u = self.up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / dims.x as f32;
        let pixel_delta_v = viewport_v / dims.y as f32;

        let upper_left =
            self.position - (self.focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;

        CameraRenderParams {
            position: self.position.into(),
            __padding0: NAN,
            upper_left: upper_left.into(),
            __padding1: NAN,
            pixel_delta_u: pixel_delta_u.into(),
            __padding2: NAN,
            pixel_delta_v: pixel_delta_v.into(),
            __padding3: NAN,
        }
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct CameraRenderParams {
    position: [f32; 3],
    __padding0: f32,
    upper_left: [f32; 3],
    __padding1: f32,
    pixel_delta_u: [f32; 3],
    __padding2: f32,
    pixel_delta_v: [f32; 3],
    __padding3: f32,
}

pub struct CameraController {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub speed: f32,
}
