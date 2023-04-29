# PyWavers
A mixed Rust and Python repository/package that exposes the Wavers Rust crate to Python. 

## Building and installing PyWavers
Firstly, PyWavers requries a Python version >= 3.10. Secondly the only main requirements is are Maturin and Numpy.

### Step 1
Create a Python venv/conda env that is running Python >= 3.10. 

### Step 2

``pip install numpy maturin``


### Step 3
From the project root:

``maturin develop --release``

### Step 4
Congratulations, PyWavers has now been installed into the environment created in [Step 1](#step-1) and can be used in your Python code.

## Example Usage
```python
import numpy as np
import pywavers as pw

if __name__ == '__main__':
    data_f32 = pywavers.read('./my_test_wav.wav' dtype=np.float32)
    pywavers.write('./my_output_test_wav.wav', data_f32, sample_rate=16000, dtype=np.int16)
```
