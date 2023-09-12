use pyo3::{pyclass, pymodule, Python, pyfunction, PyResult, PyAny, wrap_pyfunction, types::PyModule, exceptions::PyValueError};
use ractor::{ActorRef, Actor, cast, call};
use actor_lib::{CounterMsg, Counter};

#[pyclass]
#[derive(Clone)]
pub struct PyCounter {
    counter: ActorRef<CounterMsg>
}


#[pyfunction]
pub fn init_counter(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        Actor::spawn(None, Counter{}, ()).await
            .map(|(counter, _)| PyCounter{counter})
            .map_err(|err| PyValueError::new_err(err.to_string()))
    })
}

#[pyfunction]
pub fn increase_counter(py: Python, cnt: PyCounter, inc: usize) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        cast!(cnt.counter, CounterMsg::Inc(inc))
            .map_err(|err| PyValueError::new_err(err.to_string()))
    })
}

#[pyfunction]
pub fn get_counter(py: Python, cnt: PyCounter) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        call!(cnt.counter, CounterMsg::Get)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    })
}

#[pymodule]
fn actor_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_counter, m)?)?;
    m.add_function(wrap_pyfunction!(increase_counter, m)?)?;
    m.add_function(wrap_pyfunction!(get_counter, m)?)?;
    Ok(())
}