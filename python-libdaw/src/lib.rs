mod indexing;
mod metronome;
mod node;
mod nodes;
mod notation;
mod pitch;
mod play;
mod sample;
mod time;

pub use node::Node;
pub use sample::Sample;

use pyo3::{
    create_exception, exceptions::PyRuntimeError, pymodule, types::PyModule, wrap_pyfunction_bound,
    Bound, PyErr, PyResult, Python,
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
    m.add_class::<Sample>()?;
    m.add_class::<Node>()?;
    m.add_function(wrap_pyfunction_bound!(play::play, m)?)?;

    nodes::register(&submodule!(m, "libdaw", "nodes"))?;
    pitch::register(&submodule!(m, "libdaw", "pitch"))?;
    metronome::register(&submodule!(m, "libdaw", "metronome"))?;
    time::register(&submodule!(m, "libdaw", "time"))?;
    notation::register(&submodule!(m, "libdaw", "notation"))?;
    Ok(())
}
