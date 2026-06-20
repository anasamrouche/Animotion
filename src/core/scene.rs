use crate::core::{
    camera::{Camera, CameraController},
    objects::{AnimColor, AnimObject},
    time::Time,
};

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<AnimObject>,
    pub background_color: AnimColor,
    pub camera: Camera,
    pub camera_controller: CameraController,
    pub time: Time,
}

impl Scene {
    pub fn new(objects: Vec<AnimObject>, background_color: AnimColor, fps: u8) -> Self {
        Self {
            objects,
            background_color,
            camera: Camera::new(),
            camera_controller: CameraController::new(),
            time: Time::new(fps),
        }
    }

    pub(crate) fn add_object(&mut self, object: AnimObject) {
        self.objects.push(object);
    }

    pub fn background_color_f64(&self) -> [f64; 4] {
        let color = self.background_color.to_array();
        [
            color[0] as f64,
            color[1] as f64,
            color[2] as f64,
            color[3] as f64,
        ]
    }
}
