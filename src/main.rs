mod app;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();

    let _ = event_loop.run_app(&mut app);
}
