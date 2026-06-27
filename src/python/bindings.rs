use std::sync::Arc;

use pyo3::prelude::*;

use crate::core::{
    objects::{AnimColor, AnimFloat, AnimObject, AnimPosition, Line, Tetrahedron},
    scene::Scene as RustScene,
};

#[pyclass(name = "Scene")]
struct PyScene {
    scene: RustScene,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new(background_color: [f32; 4], fps: u8) -> Self {
        Self {
            scene: RustScene::new(
                vec![],
                AnimColor::from_array(background_color, Arc::from(|_| {})),
                fps,
            ),
        }
    }

    fn add_object(&mut self, object: &Bound<'_, PyAny>) -> PyResult<()> {
        let extracted_object: Result<PyAnimObject, _> = object.extract();
        match extracted_object {
            Ok(object) => {
                self.scene.add_object(object.into());
                Ok(())
            }
            Err(e) => return Err(e),
        }
    }

    fn rotate(&mut self, phi: f32, theta: f32, zoom: Option<f32>) -> PyResult<()> {
        if let Some(zoom) = zoom {
            self.scene.camera_controller.zoom(zoom);
        }
        self.scene.camera_controller.rotate(phi, theta);
        self.scene
            .camera_controller
            .update_camera(&mut self.scene.camera);

        Ok(())
    }
}

#[derive(FromPyObject)]
enum PyAnimObject {
    Line(PyLine),
    Tetrahedron(PyTetrahedron),
}

impl From<PyAnimObject> for AnimObject {
    fn from(pyobject: PyAnimObject) -> Self {
        match pyobject {
            PyAnimObject::Line(line) => AnimObject::Line(line.inner),
            PyAnimObject::Tetrahedron(tetrahedron) => AnimObject::Tetrahedron(tetrahedron.inner),
        }
    }
}

#[pyclass(from_py_object, name = "Line")]
#[derive(Clone)]
struct PyLine {
    inner: Line,
}

#[pymethods]
impl PyLine {
    #[new]
    fn new(
        start_point: (f32, f32, f32),
        end_point: (f32, f32, f32),
        start_color: (f32, f32, f32, f32),
        end_color: (f32, f32, f32, f32),
        thickness: f32,
    ) -> Self {
        Self {
            inner: Line::new(
                AnimPosition::new(
                    start_point.0,
                    start_point.1,
                    start_point.2,
                    1.0,
                    Arc::from(|_| {}),
                ),
                AnimPosition::new(
                    end_point.0,
                    end_point.1,
                    end_point.2,
                    1.0,
                    Arc::from(|_| {}),
                ),
                AnimColor::new(
                    start_color.0,
                    start_color.1,
                    start_color.2,
                    start_color.3,
                    Arc::from(|_| {}),
                ),
                AnimColor::new(
                    end_color.0,
                    end_color.1,
                    end_color.2,
                    end_color.3,
                    Arc::from(|_| {}),
                ),
                AnimFloat::new(thickness / 120.0, Arc::from(|_| {})),
            ),
        }
    }
}

#[pyclass(from_py_object, name = "Tetrahedron")]
#[derive(Clone)]
struct PyTetrahedron {
    inner: Tetrahedron,
}

#[pymethods]
impl PyTetrahedron {
    #[new]
    fn new(position: [f32; 3], color: ([f32; 4], [f32; 4], [f32; 4], [f32; 4]), size: f32) -> Self {
        Self {
            inner: Tetrahedron::new(
                AnimPosition::new(position[0], position[1], position[2], 1.0, Arc::new(|_| {})),
                (
                    AnimColor::new(
                        color.0[0],
                        color.0[1],
                        color.0[2],
                        color.0[3],
                        Arc::new(|_| {}),
                    ),
                    AnimColor::new(
                        color.1[0],
                        color.1[1],
                        color.1[2],
                        color.1[3],
                        Arc::new(|_| {}),
                    ),
                    AnimColor::new(
                        color.2[0],
                        color.2[1],
                        color.2[2],
                        color.2[3],
                        Arc::new(|_| {}),
                    ),
                    AnimColor::new(
                        color.3[0],
                        color.3[1],
                        color.3[2],
                        color.3[3],
                        Arc::new(|_| {}),
                    ),
                ),
                AnimFloat::new(size, Arc::new(|_| {})),
            ),
        }
    }
}

#[pymodule]
mod animotion {
    use super::*;
    use crate::gpu::run;

    #[pymodule_export]
    use super::{PyLine, PyScene, PyTetrahedron};

    #[pyfunction]
    fn create_scene(background_color: [f32; 4], fps: u8) -> PyResult<PyScene> {
        let scene = PyScene::new(background_color, fps);
        Ok(scene)
    }

    #[pyfunction]
    fn debug_window(scene: &Bound<'_, PyScene>) {
        let scene = scene.borrow().scene.clone();
        let _ = run(scene);
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("PI", std::f32::consts::PI)?;
        m.add("E", std::f32::consts::E)?;

        Ok(())
    }
}
