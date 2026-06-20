use std::sync::{Arc, Mutex};

pub trait DynamicObject {
    fn notify(&self, object: AnimObject, callback: Arc<dyn Fn(AnimObject) + Send + Sync>);

    fn get_value(&self) -> f32;

    fn set_value(&mut self, value: f32);
}

#[derive(Clone)]
pub struct AnimFloat {
    value: f32,
    callback: Arc<dyn Fn(AnimObject) + Send + Sync>,
    dependencies: Vec<Arc<Mutex<AnimFloat>>>,
}

impl AnimFloat {
    pub fn new(value: f32, callback: Arc<dyn Fn(AnimObject) + Send + Sync>) -> Self {
        Self {
            value,
            callback,
            dependencies: vec![],
        }
    }
}

impl DynamicObject for AnimFloat {
    fn notify(&self, object: AnimObject, callback: Arc<dyn Fn(AnimObject) + Send + Sync>) {
        callback(object);
    }

    fn get_value(&self) -> f32 {
        self.value
    }

    fn set_value(&mut self, value: f32) {
        self.value = value;
        for dependency in &self.dependencies {
            let dep = dependency.lock().unwrap();
            self.notify(AnimObject::AnimFloat(dep.clone()), self.callback.clone());
        }
    }
}

#[derive(Clone)]
pub(crate) struct AnimPosition {
    pub x: AnimFloat,
    pub y: AnimFloat,
    pub z: AnimFloat,
    pub w: AnimFloat,
    pub callback: Arc<dyn Fn(AnimObject) + Send + Sync>,
}

impl AnimPosition {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        callback: Arc<dyn Fn(AnimObject) + Send + Sync>,
    ) -> Self {
        Self {
            x: AnimFloat::new(x, Arc::from(|_| {})),
            y: AnimFloat::new(y, Arc::from(|_| {})),
            z: AnimFloat::new(z, Arc::from(|_| {})),
            w: AnimFloat::new(w, Arc::from(|_| {})),
            callback: Arc::from(callback),
        }
    }

    pub(crate) fn to_array(&self) -> [f32; 4] {
        [
            self.x.get_value(),
            self.y.get_value(),
            self.z.get_value(),
            self.w.get_value(),
        ]
    }
}

#[derive(Clone)]
pub(crate) struct AnimColor {
    r: AnimFloat,
    g: AnimFloat,
    b: AnimFloat,
    a: AnimFloat,
    callback: Arc<dyn Fn(AnimObject) + Send + Sync>,
}

impl AnimColor {
    pub fn new(
        r: f32,
        g: f32,
        b: f32,
        a: f32,
        callback: Arc<dyn Fn(AnimObject) + Send + Sync>,
    ) -> Self {
        Self {
            r: AnimFloat::new(r, Arc::from(|_| {})),
            g: AnimFloat::new(g, Arc::from(|_| {})),
            b: AnimFloat::new(b, Arc::from(|_| {})),
            a: AnimFloat::new(a, Arc::from(|_| {})),
            callback,
        }
    }

    pub fn from_array(arr: [f32; 4], callback: Arc<dyn Fn(AnimObject) + Send + Sync>) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3], callback)
    }

    pub(crate) fn to_array(&self) -> [f32; 4] {
        [
            self.r.get_value(),
            self.g.get_value(),
            self.b.get_value(),
            self.a.get_value(),
        ]
    }
}

use crate::core::AnimRender;
use crate::core::Vertex;
use crate::core::camera::Camera;
use crate::core::camera::CameraController;
use wgpu::PrimitiveTopology;

#[derive(Clone)]
pub(crate) enum AnimObject {
    #[allow(unused)]
    AnimFloat(AnimFloat),
    Line(Line),
}

#[derive(Clone)]
pub(crate) struct Line {
    pub start_point: AnimPosition,
    pub end_point: AnimPosition,
    pub start_color: AnimColor,
    pub end_color: AnimColor,
    pub thickness: AnimFloat,
}

impl AnimRender for Line {
    const TOPOLOGY: PrimitiveTopology = PrimitiveTopology::TriangleList;

    fn get_vertices(&self) -> Vec<Vertex> {
        let t = self.thickness.get_value() / 2.0;
        let dx = self.end_point.x.get_value() - self.start_point.x.get_value();
        let dy = self.end_point.y.get_value() - self.start_point.y.get_value();
        let len = (dx * dx + dy * dy).sqrt();
        let normal = [dy / len, -dx / len, 0.0, 0.0];

        let start_point = self.start_point.to_array();
        let v11 = [
            start_point[0] + normal[0] * t,
            start_point[1] + normal[1] * t,
            start_point[2] + normal[2] * t,
            start_point[3] + normal[3] * t,
        ];
        let v12 = [
            start_point[0] - normal[0] * t,
            start_point[1] - normal[1] * t,
            start_point[2] - normal[2] * t,
            start_point[3] - normal[3] * t,
        ];
        let end_point = self.end_point.to_array();
        let v21 = [
            end_point[0] + normal[0] * t,
            end_point[1] + normal[1] * t,
            end_point[2] + normal[2] * t,
            end_point[3] + normal[3] * t,
        ];
        let v22 = [
            end_point[0] - normal[0] * t,
            end_point[1] - normal[1] * t,
            end_point[2] - normal[2] * t,
            end_point[3] - normal[3] * t,
        ];

        vec![
            Vertex::new(v11, self.start_color.to_array()),
            Vertex::new(v12, self.start_color.to_array()),
            Vertex::new(v21, self.end_color.to_array()),
            Vertex::new(v22, self.end_color.to_array()),
        ]
    }

    fn get_indices(&self) -> Vec<u32> {
        Vec::from([0, 1, 2, 2, 3, 1])
    }
}

impl Line {
    pub(crate) fn new(
        start_point: AnimPosition,
        end_point: AnimPosition,
        start_color: AnimColor,
        end_color: AnimColor,
        thickness: AnimFloat,
    ) -> Self {
        Self {
            start_point,
            end_point,
            start_color,
            end_color,
            thickness,
        }
    }
}

#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<AnimObject>,
    pub background_color: AnimColor,
    pub camera: Camera,
    pub camera_controller: CameraController,
}

impl Scene {
    pub fn new(objects: Vec<AnimObject>, background_color: AnimColor) -> Self {
        Self {
            objects,
            background_color,
            camera: Camera::new(),
            camera_controller: CameraController::new(),
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
