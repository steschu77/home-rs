pub struct Animation<T> {
    t0: f32, // Start time of the animation
    t1: f32, // End time of the animation
    x0: T,   // Start value of the animation
    x1: T,   // End value of the animation
}

impl<T> Animation<T>
where
    T: Copy
        + std::ops::Sub<Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Mul<f32, Output = T>,
{
    // Create a new animation
    pub fn new(t0: f32, t1: f32, x0: T, x1: T) -> Self {
        Animation { t0, t1, x0, x1 }
    }

    // Evaluate the animation at time t
    pub fn blend(&self, t: f32) -> T {
        if t <= self.t0 {
            self.x0
        } else if t >= self.t1 {
            self.x1
        } else {
            let s = (t - self.t0) / (self.t1 - self.t0);
            self.x0 + (self.x1 - self.x0) * s
        }
    }
}
