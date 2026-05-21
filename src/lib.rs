pub mod constants;
pub mod element;
pub mod geometry;
pub mod graph;
pub mod io;
pub mod kinetics;
pub mod molecule;
pub mod pattern;
pub mod reaction;
pub mod species;
pub mod states;
pub mod thermo;
pub mod thermo_converter;

use pyo3::prelude::*;

#[pyclass]
pub struct PyElement {
    pub inner: &'static element::Element,
}

#[pymethods]
impl PyElement {
    #[getter]
    fn symbol(&self) -> &str {
        self.inner.symbol
    }
    #[getter]
    fn number(&self) -> u16 {
        self.inner.number
    }
    #[getter]
    fn name(&self) -> &str {
        self.inner.name
    }
    #[getter]
    fn mass(&self) -> f64 {
        self.inner.mass
    }
}

#[pyfunction]
fn get_element(number: Option<u16>, symbol: Option<&str>) -> PyResult<Option<PyElement>> {
    let n = number.unwrap_or(0);
    let s = symbol.unwrap_or("");
    Ok(element::get_element(n, s).map(|e| PyElement { inner: e }))
}

#[pyclass]
pub struct PyWilhoitModel {
    pub inner: thermo::WilhoitModel,
}

#[pymethods]
impl PyWilhoitModel {
    #[new]
    #[allow(clippy::too_many_arguments)]
    fn new(cp0: f64, cp_inf: f64, a0: f64, a1: f64, a2: f64, a3: f64, h0: f64, s0: f64, b: f64) -> Self {
        PyWilhoitModel {
            inner: thermo::WilhoitModel::new(cp0, cp_inf, a0, a1, a2, a3, h0, s0, b),
        }
    }

    fn get_heat_capacity(&self, t: f64) -> f64 {
        use crate::thermo::ThermoModel;
        self.inner.get_heat_capacity(t)
    }

    fn fit_to_data(
        &mut self,
        t_list: Vec<f64>,
        cp_list: Vec<f64>,
        linear: bool,
        n_freq: usize,
        n_rotors: usize,
        h298: f64,
        s298: f64,
        b0: f64,
    ) {
        self.inner.fit_to_data(&t_list, &cp_list, linear, n_freq, n_rotors, h298, s298, b0);
    }
}

#[pyclass]
pub struct PyNASAModel {
    pub inner: thermo::NASAModel,
}

#[pymethods]
impl PyNASAModel {
    fn get_heat_capacity(&self, t: f64) -> f64 {
        use crate::thermo::ThermoModel;
        self.inner.get_heat_capacity(t)
    }
}

#[pyfunction]
fn convert_wilhoit_to_nasa(
    wilhoit: &PyWilhoitModel,
    t_min: f64,
    t_max: f64,
    t_int: f64,
) -> PyNASAModel {
    PyNASAModel {
        inner: thermo_converter::convert_wilhoit_to_nasa(&wilhoit.inner, t_min, t_max, t_int, true, true, 3),
    }
}

#[pymodule]
fn chempy_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_element, m)?)?;
    m.add_function(wrap_pyfunction!(convert_wilhoit_to_nasa, m)?)?;
    m.add_class::<PyElement>()?;
    m.add_class::<PyWilhoitModel>()?;
    m.add_class::<PyNASAModel>()?;
    Ok(())
}
