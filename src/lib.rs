use std::path::Path;

use numpy::pyo3::Python;
use numpy::{Ix2, PyArray};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use wavers::{
    signal_channels, signal_duration, signal_info, signal_sample_rate, write_wav_as, SampleType,
    SignalInfo, WavFile,
};

#[pyfunction]
pub fn _read_i16(py: Python, fp: String) -> PyResult<&PyArray<i16, Ix2>> {
    let wav_fp = Path::new(&fp);

    let wav = match WavFile::from_file(wav_fp) {
        Ok(wav) => wav,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Error opening wav file: {}",
                e
            )))
        }
    };

    let wav_arr = wav
        .read(Some(SampleType::I16(0 as i16)))
        .mapv(|sample| sample.to_i16());
    Ok(PyArray::from_array(py, &wav_arr))
}

#[pyfunction]
pub fn _read_i32(py: Python, fp: String) -> PyResult<&PyArray<i32, Ix2>> {
    let wav_fp = Path::new(&fp);

    let wav = match WavFile::from_file(wav_fp) {
        Ok(wav) => wav,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Error opening wav file: {}",
                e
            )))
        }
    };

    let wav_arr = wav
        .read(Some(SampleType::I32(0 as i32)))
        .mapv(|sample| sample.to_i32());
    Ok(PyArray::from_array(py, &wav_arr))
}

#[pyfunction]
pub fn _read_f32(py: Python, fp: String) -> PyResult<&PyArray<f32, Ix2>> {
    let wav_fp = Path::new(&fp);

    let wav = match WavFile::from_file(wav_fp) {
        Ok(wav) => wav,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Error opening wav file: {}",
                e
            )))
        }
    };

    let wav_arr = wav
        .read(Some(SampleType::F32(0.0 as f32)))
        .mapv(|sample| sample.to_f32());
    Ok(PyArray::from_array(py, &wav_arr))
}

#[pyfunction]
pub fn _read_f64(py: Python, fp: String) -> PyResult<&PyArray<f64, Ix2>> {
    let wav_fp = Path::new(&fp);

    let wav = match WavFile::from_file(wav_fp) {
        Ok(wav) => wav,
        Err(e) => {
            return Err(PyValueError::new_err(format!(
                "Error opening wav file: {}",
                e
            )))
        }
    };

    let wav_arr = wav
        .read(Some(SampleType::F64(0.0 as f64)))
        .mapv(|sample| sample.to_f64());
    Ok(PyArray::from_array(py, &wav_arr))
}

#[pyfunction]
pub fn _write_wav_i16(
    out_fp: String,
    signal: &PyArray<i16, Ix2>,
    sample_rate: i32,
) -> PyResult<()> {
    let signal = unsafe { signal.as_array() };
    let signal = signal.mapv(|sample| SampleType::I16(sample));
    write_wav_as(Path::new(&out_fp), &signal, None, sample_rate)
        .map_err(|e| PyValueError::new_err(format!("Error writing wav file: {}", e)))
}

#[pyfunction]
pub fn _write_wav_i32(
    out_fp: String,
    signal: &PyArray<i32, Ix2>,
    sample_rate: i32,
) -> PyResult<()> {
    let signal = unsafe { signal.as_array() };
    let signal = signal.mapv(|sample| SampleType::I32(sample));
    write_wav_as(Path::new(&out_fp), &signal, None, sample_rate)
        .map_err(|e| PyValueError::new_err(format!("Error writing wav file: {}", e)))
}

#[pyfunction]
pub fn _write_wav_f32(
    out_fp: String,
    signal: &PyArray<f32, Ix2>,
    sample_rate: i32,
) -> PyResult<()> {
    let signal = unsafe { signal.as_array() };
    let signal = signal.mapv(|sample| SampleType::F32(sample));
    write_wav_as(Path::new(&out_fp), &signal, None, sample_rate)
        .map_err(|e| PyValueError::new_err(format!("Error writing wav file: {}", e)))
}

#[pyfunction]
pub fn _write_wav_f64(
    out_fp: String,
    signal: &PyArray<f64, Ix2>,
    sample_rate: i32,
) -> PyResult<()> {
    let signal = unsafe { signal.as_array() };
    let signal = signal.mapv(|sample| SampleType::F64(sample));
    write_wav_as(Path::new(&out_fp), &signal, None, sample_rate)
        .map_err(|e| PyValueError::new_err(format!("Error writing wav file: {}", e)))
}

#[pyfunction]
pub fn _signal_duration(fp: String) -> PyResult<u64> {
    signal_duration(Path::new(&fp)).map_err(|e| {
        PyValueError::new_err(format!(
            "Error reading wav file when getting signal duration: {}",
            e
        ))
    })
}

#[pyfunction]
pub fn _signal_channels(fp: String) -> PyResult<u16> {
    signal_channels(Path::new(&fp)).map_err(|e| {
        PyValueError::new_err(format!(
            "Error reading wav file when getting signal channels: {}",
            e
        ))
    })
}

#[pyfunction]
pub fn _signal_sample_rate(fp: String) -> PyResult<i32> {
    signal_sample_rate(Path::new(&fp)).map_err(|e| {
        PyValueError::new_err(format!(
            "Error reading wav file when getting signal sample rate: {}",
            e
        ))
    })
}

#[pyfunction]
pub fn _signal_info(fp: String) -> PyResult<SignalInfo> {
    signal_info(Path::new(&fp)).map_err(|e| {
        PyValueError::new_err(format!(
            "Error reading wav file when getting signal info: {}",
            e
        ))
    })
}

#[pymodule]
fn pywavers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_read_i16, m)?)?;
    m.add_function(wrap_pyfunction!(_read_i32, m)?)?;
    m.add_function(wrap_pyfunction!(_read_f32, m)?)?;
    m.add_function(wrap_pyfunction!(_read_f64, m)?)?;

    m.add_function(wrap_pyfunction!(_write_wav_i16, m)?)?;
    m.add_function(wrap_pyfunction!(_write_wav_i32, m)?)?;
    m.add_function(wrap_pyfunction!(_write_wav_f32, m)?)?;
    m.add_function(wrap_pyfunction!(_write_wav_f64, m)?)?;

    m.add_function(wrap_pyfunction!(_signal_duration, m)?)?;
    m.add_function(wrap_pyfunction!(_signal_channels, m)?)?;
    m.add_function(wrap_pyfunction!(_signal_sample_rate, m)?)?;
    m.add_function(wrap_pyfunction!(_signal_info, m)?)?;

    Ok(())
}
