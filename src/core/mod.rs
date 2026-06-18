use bytemuck::{Pod, Zeroable};
use wgpu::PrimitiveTopology;

pub mod objects;

pub(crate) type Point = [f32; 4];

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Vertex {
    vertices: Point,
    color: Point,
}

impl Vertex {
    pub(crate) fn new(vertices: Point, color: Point) -> Self {
        Self { vertices, color }
    }
}

pub trait AnimRender {
    const TOPOLOGY: PrimitiveTopology;
    fn get_vertices(&self) -> Vec<Vertex>;
    fn get_indices(&self) -> Vec<u32>;
}
