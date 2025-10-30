use pyo3::{pymodule};

#[pymodule]
mod jeb1pochybridpy {
    use pyo3::prelude::*;

    #[pyfunction]
    fn add(a: usize, b: usize) -> PyResult<usize> {
        Ok(jeb1pochybrid::add(a as i32, b as i32) as usize)
    }
}
