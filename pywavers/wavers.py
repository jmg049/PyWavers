from typing import Any, Dict, Tuple
import numpy as np
from pywavers import pywavers


def read(fp: str, dtype: np.dtype = np.int16) -> Tuple[np.ndarray, int]:
    """Read a WAV file and return the data as a numpy array.
    Parameters
    ----------
    fp : str
        Path to the WAV file.
    dtype : str, optional
        Data type of the returned array. Default is 'int16'.
    Returns
    -------
    data : numpy.ndarray
        Data from the WAV file.
    """
    try:
        match dtype:
            case np.int16:
                return pywavers.read_i16(fp)
            case np.int32:
                return pywavers.read_i32(fp)
            case np.float32:
                return pywavers.read_f32(fp)
            case np.float64:
                return pywavers.read_f64(fp)
            case _:
                raise ValueError(f"Unsupport dtype: {dtype}")
    except Exception as e:
        raise e


def write(fp: str, data: np.ndarray, sample_rate: int, dtype: np.dtype = np.int16):
    """Write a numpy array to a WAV file.
    Parameters
    ----------
    fp : str
        Path to the WAV file.
    data : numpy.ndarray
        Data to be written to the WAV file.
    dtype : str, optional
        Data type of the input array. Default is 'int16'.
    """
    try:
        match dtype:
            case np.int16:
                pywavers.write_i16(fp, data, sample_rate)
            case np.int32:
                pywavers.write_i32(fp, data, sample_rate)
            case np.float32:
                pywavers.write_f32(fp, data, sample_rate)
            case np.float64:
                pywavers.write_f64(fp, data, sample_rate)
            case _:
                raise ValueError(f"Unsupport dtype: {dtype}")
    except Exception as e:
        raise e


def wav_spec(fp: str) -> Dict[str, Any]:
    """Get information about a WAV file.
    Parameters
    ----------
    fp : str
        Path to the WAV file.
    Returns
    -------
    info : dict
        Dictionary containing information about the WAV file.
    """
    try:
        return pywavers.wav_spec(fp)
    except Exception as e:
        raise e


ONE_CHANNEL_I16 = "./test_resources/one_channel_i16.wav"

# For the below tests, only reading an i16 file is tested. This is due to the reflexive conversion operations used. If it can convert from i16 to i32 correctly, it can do i32 to i16 correctly as well. Same for float32 and float64


def test_read_i16_i16():
    """
    Tests reading PCM16 encoded files as PCM16. Correctness is measured against soundfile
    """
    import soundfile as sf

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.int16)

    actual_data = read(ONE_CHANNEL_I16)

    for exp, act in zip(expected_data, actual_data):
        assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_read_i16_i32():
    """
    Tests reading PCM16 encoded files as int 32. Correctness is measured against soundfile
    """
    import soundfile as sf

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.int32)

    actual_data = read(ONE_CHANNEL_I16, dtype=np.int32)

    for exp, act in zip(expected_data, actual_data):
        assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_read_i16_f32():
    """
    Tests reading PCM16 as Float 32. Correctness is measured against soundfile
    """
    import soundfile as sf
    from pytest import approx

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.float32)

    actual_data = read(ONE_CHANNEL_I16, dtype=np.float32)

    for exp, act in zip(expected_data, actual_data):
        assert exp == approx(act, 1e-4), "Data mismatch, {} != {}".format(exp, act)


def test_read_i16_f64():
    """
    Tests reading PCM16 as Float 64. Correctness is measured against soundfile
    """
    import soundfile as sf
    from pytest import approx

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.float64)

    actual_data = read(ONE_CHANNEL_I16, dtype=np.float64)

    for exp, act in zip(expected_data, actual_data):
        assert exp == approx(act, 1e-4), "Data mismatch, {} != {}".format(exp, act)


def test_write_i16():
    """
    Tests writing PCM16 encoded files as PCM16. Correctness is measured against soundfile
    """
    import soundfile as sf
    import tempfile
    import os

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.int16, always_2d=True)

    with tempfile.NamedTemporaryFile(suffix=".wav") as fp:
        write(fp.name, expected_data, sr, dtype=np.int16)

        actual_data, actual_sr = sf.read(fp.name, dtype=np.int16)
        assert sr == actual_sr, "Sample rate mismatch, {} != {}".format(sr, actual_sr)
        for exp, act in zip(expected_data, actual_data):
            assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_write_i32():
    """
    Tests writing PCM16 encoded files as int 32. Correctness is measured against soundfile
    """
    import soundfile as sf
    import tempfile
    import os

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.int32, always_2d=True)

    with tempfile.NamedTemporaryFile(suffix=".wav") as fp:
        write(fp.name, expected_data, sr, dtype=np.int32)

        actual_data, actual_sr = sf.read(fp.name, dtype=np.int32)
        assert sr == actual_sr, "Sample rate mismatch, {} != {}".format(sr, actual_sr)
        for exp, act in zip(expected_data, actual_data):
            assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_write_f32():
    """
    Tests writing PCM16 encoded files as float 32. Correctness is measured against soundfile
    """
    import soundfile as sf
    import tempfile
    import os

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.float32, always_2d=True)

    with tempfile.NamedTemporaryFile(suffix=".wav") as fp:
        write(fp.name, expected_data, sr, dtype=np.float32)

        actual_data, actual_sr = sf.read(fp.name, dtype=np.float32)
        assert sr == actual_sr, "Sample rate mismatch, {} != {}".format(sr, actual_sr)
        for exp, act in zip(expected_data, actual_data):
            assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_write_f64():
    """
    Tests writing PCM16 encoded files as float 64. Correctness is measured against soundfile
    """
    import soundfile as sf
    import tempfile
    import os

    expected_data, sr = sf.read(ONE_CHANNEL_I16, dtype=np.float64, always_2d=True)

    with tempfile.NamedTemporaryFile(suffix=".wav") as fp:
        write(fp.name, expected_data, sr, dtype=np.float64)

        actual_data, actual_sr = sf.read(fp.name, dtype=np.float64)
        assert sr == actual_sr, "Sample rate mismatch, {} != {}".format(sr, actual_sr)
        for exp, act in zip(expected_data, actual_data):
            assert exp == act, "Data mismatch, {} != {}".format(exp, act)


def test_info():
    """
    Tests the wav_spec function
    """
    expected_sr = 16000
    expected_channels = 1
    expected_duration = 10

    info = wav_spec(ONE_CHANNEL_I16)

    assert (
        info.fmt_data.sample_rate == expected_sr
    ), "Sample rate mismatch, {} != {}".format(info.fmt_data.sample_rate, expected_sr)

    assert (
        info.fmt_data.channels == expected_channels
    ), "Number of channels mismatch, {} != {}".format(
        info.fmt_data.channels, expected_channels
    )

    assert info.duration == expected_duration, "Duration mismatch, {} != {}".format(
        info.duration, expected_duration
    )
