use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};
use rayon::prelude::*;

fn fibo(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

#[pyfunction]
fn recur_fibo(n: u64) -> PyResult<u64> {
    Ok(fibo(n))
}

#[pyfunction]
fn fibo_range(n: u64) -> PyResult<Vec<u64>> {
    let targets: Vec<u64> = (0 .. n).collect();
    let results: Vec<u64> = targets
        .par_iter()
        .map(|n| fibo(*n))
        .collect();
    Ok(results)
}

#[pymodule]
fn librfib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(recur_fibo))?;
    m.add_wrapped(wrap_pyfunction!(fibo_range))?;
    Ok(())
}
