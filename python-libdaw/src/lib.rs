mod frequency_node;
mod metronome;
mod node;
mod nodes;
mod notation;
mod pitch;
mod play;
mod stream;
mod time;

pub use frequency_node::FrequencyNode;
pub use node::Node;
pub use stream::Stream;

use pyo3::{
    create_exception,
    exceptions::{PyIndexError, PyRuntimeError},
    pymodule,
    types::PyModule,
    wrap_pyfunction_bound, Bound, PyErr, PyResult, Python,
};

create_exception!(libdaw, Error, PyRuntimeError);

/// An intermediate conversion type that allows converting all Errors to our error type.
pub struct ErrorWrapper(String);

impl<T> From<T> for ErrorWrapper
where
    T: ToString,
{
    fn from(value: T) -> Self {
        ErrorWrapper(value.to_string())
    }
}

impl From<ErrorWrapper> for PyErr {
    fn from(value: ErrorWrapper) -> Self {
        Error::new_err(value.0)
    }
}

pub type Result<T> = std::result::Result<T, ErrorWrapper>;

/// Define a submodule, adding it to sys.modules.
macro_rules! submodule {
    ($parent:expr, $parent_package:literal, $name:literal) => {{
        use pyo3::types::{PyAnyMethods as _, PyModule, PyModuleMethods as _};
        let qualname = std::concat!($parent_package, '.', $name);

        let module = PyModule::new_bound($parent.py(), qualname)?;
        $parent.add($name, &module)?;
        $parent
            .py()
            .import_bound("sys")?
            .getattr("modules")?
            .set_item(qualname, module.clone())?;

        module
    }};
}

use submodule;

#[pymodule]
fn libdaw(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("Error", py.get_type_bound::<Error>())?;
    m.add_class::<Stream>()?;
    m.add_class::<Node>()?;
    m.add_class::<FrequencyNode>()?;
    m.add_function(wrap_pyfunction_bound!(play::play, m)?)?;

    nodes::register(&submodule!(m, "libdaw", "nodes"))?;
    pitch::register(&submodule!(m, "libdaw", "pitch"))?;
    metronome::register(&submodule!(m, "libdaw", "metronome"))?;
    time::register(&submodule!(m, "libdaw", "time"))?;
    notation::register(&submodule!(m, "libdaw", "notation"))?;
    Ok(())
}

/// Resolve an index, with the given length.  The resolved index must be in the
/// range [0, len).  The result will error if it ends up being out of bounds.
fn resolve_index(len: usize, index: isize) -> PyResult<usize> {
    let len = isize::try_from(len).map_err(|error| PyIndexError::new_err(error.to_string()))?;
    let index = if index < 0 { len + index } else { index };

    usize::try_from(index).map_err(|error| PyIndexError::new_err(error.to_string()))
}

/// Resolve an index, with the given length.  The result is clamped [0, len]
/// rather than erroring when negative or above len. len is still used to
/// resolve negative indexes. It will still error if the length is huge, which
/// shouldn't be possible anyway.
fn resolve_index_for_insert(len: usize, index: isize) -> PyResult<usize> {
    let len = isize::try_from(len).map_err(|error| PyIndexError::new_err(error.to_string()))?;
    let index = if index < 0 { len + index } else { index };
    Ok(index.clamp(0, len) as usize)
}
