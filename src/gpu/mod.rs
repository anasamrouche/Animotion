use crate::{core::objects::Scene, gpu::view::App};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use winit::{self, event_loop::EventLoop};

pub mod context;
pub mod renderer;
pub mod view;
pub mod wgpuctl;

pub fn run(scene: Scene) -> PyResult<()> {
    env_logger::init();

    let build_event_loop = EventLoop::builder().build();
    match build_event_loop {
        Ok(event_loop) => {
            let mut app = App::new(scene);
            let _ = event_loop.run_app(&mut app);
            return Ok(());
        }
        Err(err) => Err(PyRuntimeError::new_err(format!(
            "Échec de création de l'EventLoop : {}",
            err
        ))),
    }
}
