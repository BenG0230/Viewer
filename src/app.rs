use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 160;

pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Algorithm viewer"))
            .unwrap();

        let window_size = window.inner_size();

        let window_arc = Arc::new(window);

        let surface =
            SurfaceTexture::new(window_size.width, window_size.height, window_arc.clone());
        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();

        self.window = Some(window_arc);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                if new_size.width == 0 || new_size.height == 0 {
                    return;
                }

                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(new_size.width, new_size.height);
                    let _ = pixels.resize_buffer(new_size.width, new_size.height);
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for chunk in frame.chunks_exact_mut(4) {
                        chunk[0] = 0xFF;
                        chunk[1] = 0xFF;
                        chunk[2] = 0x00;
                        chunk[3] = 0xFF;
                    }

                    if let Err(e) = pixels.render() {
                        eprintln!("Pixels render failed: {e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.repeat {
                    // Ignore repeated presses when key is held
                    return;
                }

                if event.logical_key == Key::Named(NamedKey::Escape) {
                    event_loop.exit();
                }

                // Handle viewer controls
                // space = play/pause
                // left/right key = scrub through playback
                // up/down key = change playback speed / scrub distance?
            }

            _ => {}
        }
    }
}
