# Classes

You can define full Rust types, and access them as classes in Python. Here's a Rust class:

(See `code/python/rclass` for details)

```rust
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
```

Now you can use the class directly from Python:

```python
from librclass import MyClass

data = MyClass(12)
print(data.data)
data.print()
```

There are ways to use inheritance and similar in "advanced topics" on the [PyO3 Documentation Guide](https://pyo3.rs/main/class).