use pyo3::prelude::*;

#[pyfunction]
pub fn addf(a: f64, b: f64) -> PyResult<f64> {
    Ok(a + b)
}

#[pyfunction]
pub fn addi(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

#[pyfunction]
pub fn subf(a: f64, b: f64) -> PyResult<f64> {
    Ok(a - b)
}

#[pyfunction]
pub fn subi(a: i64, b: i64) -> PyResult<i64> {
    Ok(a - b)
}

#[pyfunction]
pub fn mulf(a: f64, b: f64) -> PyResult<f64> {
    Ok(a * b)
}

#[pyfunction]
pub fn muli(a: i64, b: i64) -> PyResult<i64> {
    Ok(a * b)
}

#[pyfunction]
pub fn truediv(a: f64, b: f64) -> PyResult<f64> {
    Ok(a / b)
}

#[pyfunction]
pub fn floordiv(a: f64, b: f64) -> PyResult<i64> {
    Ok((a / b) as i64)
}

#[pyfunction]
pub fn modulof(a: f64, b: f64) -> PyResult<f64> {
    Ok(a % b)
}

#[pyfunction]
pub fn moduloi(a: i64, b: i64) -> PyResult<i64> {
    Ok(a % b)
}

#[pyfunction]
pub fn powf(a: f64, b: f64) -> PyResult<f64> {
    Ok(a.powf(b))
}

#[pyfunction]
pub fn powi(a: i64, b: i64) -> PyResult<i64> {
    Ok((a as f64).powf(b as f64) as i64)
}

#[pymodule]
fn pyjsmath(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(addf, m)?)?;
    m.add_function(wrap_pyfunction!(addi, m)?)?;

    m.add_function(wrap_pyfunction!(subf, m)?)?;
    m.add_function(wrap_pyfunction!(subi, m)?)?;

    m.add_function(wrap_pyfunction!(mulf, m)?)?;
    m.add_function(wrap_pyfunction!(muli, m)?)?;

    m.add_function(wrap_pyfunction!(truediv, m)?)?;
    m.add_function(wrap_pyfunction!(floordiv, m)?)?;

    m.add_function(wrap_pyfunction!(modulof, m)?)?;
    m.add_function(wrap_pyfunction!(moduloi, m)?)?;

    m.add_function(wrap_pyfunction!(powf, m)?)?;
    m.add_function(wrap_pyfunction!(powi, m)?)?;
    Ok(())
}
