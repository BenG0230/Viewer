use crate::gfx::Frame;

// Playback manager for the app
// implemented on a algorithm family basis so they can use different drawers
pub trait Playback {
    fn forward(&mut self);
    fn back(&mut self);
    fn draw(&self, frame: &mut Frame);
}
