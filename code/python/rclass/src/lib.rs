use pyo3::prelude::*;

#[pyclass]
struct MyClass {
    #[pyo3(get)]
    data: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    fn new(data: i32) -> Self {
        MyClass { data }
    }

    fn print(&self) {
        println!("Data: {}", self.data);
    }
}

#[pymodule]
fn librclass(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    Ok(())
}