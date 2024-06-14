pub mod band_pass;
pub mod band_stop;
pub mod high_pass;
pub mod low_pass;

pub use band_pass::BandPass;
pub use band_stop::BandStop;
pub use high_pass::HighPass;
pub use low_pass::LowPass;

use pyo3::{
    types::{PyModule, PyModuleMethods as _},
    Bound, PyResult,
};

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<LowPass>()?;
    module.add_class::<HighPass>()?;
    module.add_class::<BandPass>()?;
    module.add_class::<BandStop>()?;
    Ok(())
}
