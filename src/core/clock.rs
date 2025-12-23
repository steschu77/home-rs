use crate::core::IClock;

// ----------------------------------------------------------------------------
pub struct Clock {}

impl IClock for Clock {
    // ------------------------------------------------------------------------
    fn t_now(&self) -> std::time::Instant {
        std::time::Instant::now()
    }

    // ------------------------------------------------------------------------
    fn dt_since(&self, t: std::time::Instant) -> std::time::Duration {
        self.t_now().duration_since(t)
    }

    // ------------------------------------------------------------------------
    fn sleep(&self, dt: std::time::Duration) -> std::time::Instant {
        std::thread::sleep(dt);
        self.t_now()
    }
}

// ----------------------------------------------------------------------------
impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

// ----------------------------------------------------------------------------
impl Clock {
    pub fn new() -> Self {
        Clock {}
    }
}
