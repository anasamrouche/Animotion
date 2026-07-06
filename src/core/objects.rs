use crate::core::{AnimRender, Vertex};
use std::{
    f32::consts::{PI, TAU},
    sync::{Arc, Mutex},
};
use wgpu::PrimitiveTopology;

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

#[derive(Clone)]
pub(crate) enum AnimObject {
    #[allow(unused)]
    AnimFloat(AnimFloat),
    Line(Line),
    Sphere(Sphere),
    Tetrahedron(Tetrahedron),
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
pub struct Tetrahedron {
    position: AnimPosition,
    color: (AnimColor, AnimColor, AnimColor, AnimColor),
    size: AnimFloat,
}

impl Tetrahedron {
    pub fn new(
        position: AnimPosition,
        color: (AnimColor, AnimColor, AnimColor, AnimColor),
        size: AnimFloat,
    ) -> Self {
        Self {
            position,
            color: (color.0, color.1, color.2, color.3),
            size,
        }
    }
}

impl AnimRender for Tetrahedron {
    const TOPOLOGY: PrimitiveTopology = PrimitiveTopology::TriangleList;

    fn get_vertices(&self) -> Vec<Vertex> {
        let s = self.size.get_value();
        let s2 = s / 2.0f32.sqrt();
        let center = self.position.to_array();

        let pos1 = [center[0] + s2, center[1] + s2, center[2] + s2, center[3]];
        let pos2 = [center[0] - s2, center[1] + s2, center[2] + s2, center[3]];
        let pos3 = [center[0], center[1] - s2, center[2] + s2, center[3]];

        vec![
            Vertex::new(pos1, self.color.0.to_array()),
            Vertex::new(pos2, self.color.1.to_array()),
            Vertex::new(pos3, self.color.2.to_array()),
            Vertex::new(center, self.color.3.to_array()),
        ]
    }

    fn get_indices(&self) -> Vec<u32> {
        Vec::from([0, 1, 3, 0, 2, 3, 1, 2, 3, 1, 3, 0])
    }
}

#[derive(Clone)]
pub struct Sphere {
    position: AnimPosition,
    radius: AnimFloat,
    color: AnimColor,
    resolution: u32,
}

impl Sphere {
    pub fn new(position: [f32; 4], radius: f32, color: [f32; 4], resolution: u32) -> Self {
        Self {
            position: AnimPosition::new(
                position[0],
                position[1],
                position[2],
                position[3],
                Arc::from(|_| {}),
            ),
            radius: AnimFloat::new(radius, Arc::from(|_| {})),
            color: AnimColor::new(color[0], color[1], color[2], color[3], Arc::new(|_| {})),
            resolution,
        }
    }
}

impl AnimRender for Sphere {
    const TOPOLOGY: PrimitiveTopology = PrimitiveTopology::TriangleList;

    fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for i in 0..=self.resolution {
            let v = i as f32 / self.resolution as f32;
            let phi = v * PI;

            for j in 0..=self.resolution {
                let u = j as f32 / self.resolution as f32;
                let theta = u * TAU;

                let x = self.radius.get_value() * phi.sin() * theta.cos();
                let y = self.radius.get_value() * phi.cos();
                let z = self.radius.get_value() * phi.sin() * theta.sin();

                vertices.push(Vertex::new([x, y, z, 1.0], self.color.to_array()));
            }
        }
        vertices
    }

    fn get_indices(&self) -> Vec<u32> {
        let mut indices = Vec::new();
        for i in 0..self.resolution {
            for j in 0..self.resolution {
                let p1 = i * (self.resolution + 1) + j;
                let p2 = p1 + (self.resolution + 1);

                if i != 0 {
                    indices.push(p1);
                    indices.push(p2);
                    indices.push(p1 + 1);
                }

                if i != self.resolution - 1 {
                    indices.push(p1 + 1);
                    indices.push(p2);
                    indices.push(p2 + 1);
                }
            }
        }
        indices
    }
}
