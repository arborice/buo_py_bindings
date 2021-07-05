#![allow(dead_code)]

use buo::*;
use pyo3::{
    exceptions::{PyFileNotFoundError, PyRuntimeError},
    prelude::*,
    wrap_pyfunction,
};
use std::path::Path;

#[pymodule]
fn buo_util(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(query_buo, m)?).unwrap();
    m.add_class::<PyMediaMeta>()?;
    m.add_class::<PyDirMeta>()?;
    Ok(())
}

fn anyhow_to_pyerr<E: ToString>(err: E) -> PyErr {
    PyRuntimeError::new_err(err.to_string())
}

#[pyfunction]
fn query_buo(_py: Python, query: String) -> PyResult<(Option<PyDirMeta>, Option<PyMediaMeta>)> {
    let path: &Path = query.as_ref();
    if !path.exists() {
        return Err(PyFileNotFoundError::new_err("Not a supported file type"));
    }

    let (mut dir_meta, mut media_meta) = (None, None);

    if path.is_dir() {
        dir_meta.replace(buo_dir_meta(path).map_err(anyhow_to_pyerr)?.into());
    } else {
        media_meta.replace(buo_media_query(path).map_err(anyhow_to_pyerr)?.into());
    }
    Ok((dir_meta, media_meta))
}

#[pyclass(name = "DirMeta", dict)]
struct PyDirMeta {
    path: String,
    disk_size: String,
    num_files: u64,
}

impl From<DirMeta> for PyDirMeta {
    fn from(
        DirMeta {
            path,
            disk_size,
            num_files,
        }: DirMeta,
    ) -> Self {
        Self {
            path: path.to_string_lossy().to_string(),
            disk_size,
            num_files,
        }
    }
}

#[pyclass(name = "MediaMeta", dict)]
struct PyMediaMeta {
    file_name: String,
    title: Option<String>,
    author: Option<String>,
    duration: Option<String>,
    media_date: Option<String>,
    stats: Option<String>,
    display_extra: bool,
    extra: Option<String>,
}

impl From<MediaMeta> for PyMediaMeta {
    fn from(
        MediaMeta {
            file_name,
            title,
            author,
            duration,
            date,
            stats,
            display_extra,
            extra,
        }: MediaMeta,
    ) -> Self {
        Self {
            file_name,
            title,
            author,
            duration: duration.map(|d| format!("{:?}", d)),
            media_date: date.map(|d| d.to_string()),
            stats: stats.map(|mut s| {
                s.drain(..)
                    .map(|stat| stat.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            }),
            display_extra,
            extra,
        }
    }
}
