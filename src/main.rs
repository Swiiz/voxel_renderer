use voxel_renderer::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    //std::env::set_var("RUST_BACKTRACE", "1");
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut App::default()).unwrap();
}
