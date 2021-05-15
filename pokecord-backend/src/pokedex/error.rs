use pyo3::prelude::*;
use pyo3::PyErrArguments;

/// PokeAPI error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP request failed")]
    Http(#[from] reqwest::Error),
    #[error("fs access failed")]
    Filesystem(#[from] std::io::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("Cache serialization failed")]
    CacheSerialization(#[from] bincode::Error),
}

impl PyErrArguments for Error {
    fn arguments(self, py: Python) -> PyObject {
        self.to_string().into_py(py)
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        crate::PokedexError::new_err(err)
    }
}
