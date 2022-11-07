import string_sum

# First run `maturin develop` to build the extension module
# and then run `python main.py` to run this file

# Or you can manually build the extension module https://pyo3.rs/v0.15.1/building_and_distribution.html#manual-builds

print(string_sum.sum_as_string(5, 20))