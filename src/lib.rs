use wavers::{i24, read, write, AsNdarray, IntoNdarray, Wav, WavHeader, WavType};

use numpy::pyo3::Python;
use numpy::{IntoPyArray, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

macro_rules! _read {
    ($($T:ident), *) => {
        $(
            paste::item! {
                #[pyfunction]
                pub fn [<read_ $T>](py: Python, fp: String) -> PyResult<(&PyArray2<$T>, i32)> {
                    let wav: Wav<$T> = Wav::<$T>::from_path(&fp).map_err(|e| PyValueError::new_err(format!("Error opening wav file: {}", e)))?;

                    let (samples, sample_rate) = wav.into_ndarray().map_err(|e| PyValueError::new_err(format!("Error converting wav file to ndarray: {}", e)))?;

                    Ok((samples.into_pyarray(py), sample_rate))
                }
            }
        )*
    };
}

_read!(i16, i24, i32, f32, f64);

macro_rules! _write {
    ($($T:ident), *) => {
        $(
            paste::item! {
                #[pyfunction]
                pub fn [<write_$T>](_py: Python, fp: String, data: &PyArray2<$T>, sample_rate: u32) -> PyResult<()> {
                    let slice_data: &[$T] = unsafe {
                        data.as_slice()?
                    };
                    write::<$T, &str>(&fp, slice_data, sample_rate as i32, data.shape()[1] as u16)
                        .map_err(|e| PyValueError::new_err(format!("Error writing wav file: {}", e)))
                }
            }
        )*
    };
}

_write!(i16, i24, i32, f32, f64);

#[pyclass]
pub struct WavSpec {
    #[pyo3(get)]
    pub duration: u32,
    #[pyo3(get)]
    pub header: WavHeader,
}

#[pyfunction]
pub fn wav_spec(fp: String) -> PyResult<WavSpec> {
    let (duration, header) = wavers::wav_spec(&fp)
        .map_err(|e| PyValueError::new_err(format!("Error reading wav file: {}", e)))?;
    Ok(WavSpec { duration, header })
}

#[pymodule]
fn pywavers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WavSpec>()?;
    m.add_function(wrap_pyfunction!(wav_spec, m)?)?;
    m.add_function(wrap_pyfunction!(read_i16, m)?)?;
    m.add_function(wrap_pyfunction!(read_i24, m)?)?;
    m.add_function(wrap_pyfunction!(read_i32, m)?)?;
    m.add_function(wrap_pyfunction!(read_f32, m)?)?;
    m.add_function(wrap_pyfunction!(read_f64, m)?)?;

    m.add_function(wrap_pyfunction!(write_i16, m)?)?;
    m.add_function(wrap_pyfunction!(write_i24, m)?)?;
    m.add_function(wrap_pyfunction!(write_i32, m)?)?;
    m.add_function(wrap_pyfunction!(write_f32, m)?)?;
    m.add_function(wrap_pyfunction!(write_f64, m)?)?;
    Ok(())
}
