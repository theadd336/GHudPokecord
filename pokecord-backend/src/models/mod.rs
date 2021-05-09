use pyo3::prelude::*;

pub fn init_submodule(module: &PyModule) -> PyResult<()> {
    module.add_class::<Pokemon>();
    Ok(())
}

#[pyclass]
pub struct Pokemon {}
