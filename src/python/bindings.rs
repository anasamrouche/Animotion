use pyo3::prelude::*;

use crate::{
    core::objects::objects::Scene as RustScene,
    core::objects::{
        objects::{AnimObject, Line},
        primitives::{AnimColor, AnimFloat, AnimPosition},
    },
};

#[pyclass(name = "Scene")]
struct PyScene {
    scene: RustScene,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new(background_color: [f32; 4]) -> Self {
        Self {
            scene: RustScene::new(vec![], AnimColor::from_array(background_color)),
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
}

impl From<PyAnimObject> for AnimObject {
    fn from(pyobject: PyAnimObject) -> Self {
        match pyobject {
            PyAnimObject::Line(line) => AnimObject::Line(line.inner),
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
                AnimPosition::new(start_point.0, start_point.1, start_point.2, 1.0),
                AnimPosition::new(end_point.0, end_point.1, end_point.2, 1.0),
                AnimColor::new(start_color.0, start_color.1, start_color.2, start_color.3),
                AnimColor::new(end_color.0, end_color.1, end_color.2, end_color.3),
                AnimFloat::new(thickness / 120.0),
            ),
        }
    }
}

#[pymodule]
mod animotion {
    use super::*;
    use crate::gpu::run;

    #[pymodule_export]
    use super::{PyLine, PyScene};

    #[pyfunction]
    fn create_scene(background_color: [f32; 4]) -> PyResult<PyScene> {
        let scene = PyScene::new(background_color);
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
