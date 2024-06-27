use std::{sync::Arc, time::Instant};

use graphics::{camera::Camera, Graphics};
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, MouseScrollDelta, RawKeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

mod graphics;
mod maths;

#[derive(Default)]
pub enum App {
    #[default]
    Init,
    Running {
        start_time: Instant,
        last_update: Instant,
        window: Arc<Window>,
        graphics: Graphics<'static>,

        camera: Camera,
    },
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let graphics = Graphics::new(window.inner_size(), window.clone());

        *self = Self::Running {
            start_time: Instant::now(),
            last_update: Instant::now(),
            window,
            graphics,
            camera: Camera::default(),
        };
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match (event, self) {
            (WindowEvent::CloseRequested, _) => {
                event_loop.exit();
            }
            (
                WindowEvent::RedrawRequested,
                Self::Running {
                    window,
                    graphics,
                    start_time,
                    camera,
                    ..
                },
            ) => {
                let time: f32 = 0.5 + start_time.elapsed().as_micros() as f32 * 1e-6;
                graphics.render(camera, time);
                window.request_redraw();
            }

            (
                WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. },
                Self::Running {
                    graphics, window, ..
                },
            ) => graphics.resize(window.inner_size()),
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match (event, self) {
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::F5),
                    ..
                }),
                Self::Running { graphics, .. },
            ) => graphics.refresh(),
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyW),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.forward = state != ElementState::Released,
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyS),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.backward = state != ElementState::Released,
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyA),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.left = state != ElementState::Released,
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::KeyD),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.right = state != ElementState::Released,
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::Space),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.up = state != ElementState::Released,
            (
                DeviceEvent::Key(RawKeyEvent {
                    physical_key: PhysicalKey::Code(KeyCode::ShiftLeft),
                    state,
                    ..
                }),
                Self::Running { camera, .. },
            ) => camera.controller.down = state != ElementState::Released,
            (DeviceEvent::MouseWheel { delta, .. }, Self::Running { camera, .. }) => match delta {
                MouseScrollDelta::LineDelta(_, y) => camera.controller.speed += 0.1 * y,
                MouseScrollDelta::PixelDelta(pos) => camera.controller.speed += 0.1 * pos.y as f32,
            },
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Self::Running {
            last_update,
            camera,
            ..
        } = self
        {
            let dt = last_update.elapsed().as_secs_f32();
            *last_update = Instant::now();

            camera.update_movement(dt);
        }
    }
}
