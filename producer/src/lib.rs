use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct MyClass {
    data: u64,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(data: u64) -> Self {
        MyClass { data }
    }

    pub fn get_data(&self) -> u64 {
        self.data
    }
}

#[pymodule]
fn producer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}
