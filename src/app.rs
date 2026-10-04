use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowId},
};

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

        let window_arc = Arc::new(window);

        let surface = SurfaceTexture::new(160, 160, window_arc.clone());
        let pixels = Pixels::new(160, 160, surface).unwrap();

        self.window = Some(window_arc);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    pixels.render().unwrap();
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
