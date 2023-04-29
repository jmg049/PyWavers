import numpy as np
from pywavers import pywavers


def read(fp: str, dtype: np.dtype = np.int16) -> np.ndarray :
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
                return pywavers._read_i16(fp)
            case np.int32:
                return pywavers._read_i32(fp)
            case np.float32:
                return pywavers._read_f32(fp)
            case np.float64:
                return pywavers._read_f64(fp)
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
                pywavers._write_wav_i16(fp, data, sample_rate)
            case np.int32:
                pywavers._write_wav_i32(fp, data, sample_rate)
            case np.float32:
                pywavers._write_wav_f32(fp, data, sample_rate)
            case np.float64:
                pywavers._write_wav_f64(fp, data, sample_rate)
            case _:
                raise ValueError(f"Unsupport dtype: {dtype}")
    except Exception as e:
        raise e