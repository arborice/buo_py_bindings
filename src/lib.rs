use buo::*;
use pyo3::{
    exceptions::{PyFileNotFoundError, PyRuntimeError},
    prelude::*,
    wrap_pyfunction,
};
use std::path::Path;

#[pymodule]
fn buo_py_bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_buo_output, m)?)?;
    Ok(())
}

fn anyhow_to_pyerr<E: ToString>(err: E) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyfunction]
pub fn get_buo_output(_py: Python, query: String) -> PyResult<String> {
    let path: &Path = query.as_ref();
    if !path.exists() {
        return Err(PyFileNotFoundError::new_err("Not a supported file type"));
    }

    if path.is_dir() {
        let meta: ExportedJson<_> = buo_dir_meta(path).map_err(anyhow_to_pyerr)?.into();
        let res = meta.print().map_err(anyhow_to_pyerr)?;
        Ok(res)
    } else {
        let meta: ExportedJson<_> = buo_media_query(path).map_err(anyhow_to_pyerr)?.into();
        let res = meta.print().map_err(anyhow_to_pyerr)?;
        Ok(res)
    }
}
