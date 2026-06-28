use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use std::{f32::consts::PI, num::FpCategory::Nan};

#[derive(Clone)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            eye: Vec3::new(0.0, 0.0, 5.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            aspect: 1.0,
            fov: PI / 4.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn build_view_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_infinite_rh(self.fov, self.aspect, self.znear);
        proj * view
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CameraUniform {
    pub camera: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            camera: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view(&mut self, camera: &Camera) {
        self.camera = camera.build_view_matrix().to_cols_array_2d();
    }
}

#[derive(Clone)]
pub struct CameraController {
    pub radius: f32,
    pub theta: f32,
    pub phi: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            radius: 5.0,
            phi: PI / 2.0,
            theta: PI / 2.0,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let safe_phi = self.phi.clamp(0.01, PI - 0.01);

        let x = self.radius * safe_phi.sin() * self.theta.cos();
        let y = self.radius * safe_phi.cos();
        let z = self.radius * safe_phi.sin() * self.theta.sin();

        camera.eye = camera.target + Vec3::new(x, y, z);
    }

    pub fn rotate(&mut self, delta_phi: f32, delta_theta: f32) {
        self.theta += delta_theta;
        self.phi += delta_phi;
    }

    pub fn zoom(&mut self, amount: f32) {
        self.radius -= amount;
        self.radius = self.radius.clamp(0.1, 100.0);
    }
}
