#[derive(Clone)]
pub struct Time {
    value: f32,
    fps: u8,
}

impl Time {
    pub fn new(fps: u8) -> Self {
        Self { value: 0.0, fps }
    }

    pub fn update(&mut self, dt: f32) {
        self.value += dt;
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn fps(&self) -> u8 {
        self.fps
    }
}
