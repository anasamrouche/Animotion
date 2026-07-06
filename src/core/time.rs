use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Time {
    value: f32,
    fps: u32,
}

impl Time {
    pub fn new(fps: u32) -> Self {
        Self { value: 0.0, fps }
    }

    pub fn update(&mut self, dt: f32) {
        self.value += dt;
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }
}
