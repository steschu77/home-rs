use crate::error::Result;

pub mod app_loop;
pub mod camera;
pub mod clock;
pub mod gl_canvas;
pub mod gl_graphics;
pub mod gl_pipeline;
pub mod gl_renderer;
pub mod input;

// ----------------------------------------------------------------------------
pub trait IClock {
    fn t_now(&self) -> std::time::Instant;
    fn dt_since(&self, t: std::time::Instant) -> std::time::Duration;
    fn sleep(&self, dt: std::time::Duration) -> std::time::Instant;
}

// ----------------------------------------------------------------------------
pub trait IApp {
    fn update(
        &mut self,
        t: std::time::Instant,
        dt: std::time::Duration,
        input: &mut input::Input,
    ) -> Result<()>;
    fn render(&mut self, t: &std::time::Instant) -> Result<()>;
}
