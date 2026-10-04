mod app;
mod catalog;
mod families;
mod gfx;
mod playback;

use app::App;
use winit::event_loop::EventLoop;

fn main() -> Result<(), winit::error::EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app)
}
