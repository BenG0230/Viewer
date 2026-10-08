use crate::gfx::Frame;

// Playback manager for the app
// implemented on a algorithm family basis so they can use different drawers
pub trait Playback {
    fn jump(&mut self, dx: i32) {
        if dx == 0 {
            return;
        }

        for _ in 0..dx.abs() as usize {
            if dx > 0 {
                self.forward();
            } else {
                self.back();
            }
        }
    }
    fn forward(&mut self);
    fn back(&mut self);
    fn draw(&self, frame: &mut Frame);
}
