use crate::core::{IApp, IClock, input};
use crate::error::Result;

// --------------------------------------------------------------------------------
pub struct AppLoop {
    dt_update: std::time::Duration,
    t_lag: std::time::Duration,
}

impl AppLoop {
    // ----------------------------------------------------------------------------
    pub fn new(dt_update: std::time::Duration) -> Self {
        Self {
            dt_update,
            t_lag: std::time::Duration::ZERO,
        }
    }

    // ----------------------------------------------------------------------------
    pub fn step<App: IApp, Clock: IClock>(
        &mut self,
        app: &mut App,
        clock: &Clock,
        input: &mut input::Input,
    ) -> Result<()> {
        // generic app loop: https://gameprogrammingpatterns.com/game-loop.html
        // Goal: consume dt_update time in this step, sleep if ahead, catch up if behind
        let t0 = clock.t_now();

        // Slow machines: Clamp number of updates to avoid spiral of death
        // (otherwise the next loop will be late again)
        let updates_needed = (self.t_lag.as_nanos() / self.dt_update.as_nanos()) as u32 + 1;
        for _ in 0..updates_needed.min(4) {
            app.update(t0, self.dt_update, input)?;
        }

        app.render(&t0)?;

        self.t_lag += clock.dt_since(t0);

        if let Some(t_sleep) = self.dt_update.checked_sub(self.t_lag) {
            // Fast machines: sleep to maintain a consistent update rate
            clock.sleep(t_sleep);
        }

        // Pretend that all updates have been processed
        self.t_lag = self.t_lag.saturating_sub(self.dt_update * updates_needed);
        Ok(())
    }
}
