use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use producer::MyClass;

#[pyfunction]
fn print_data(cls: &MyClass) {
    println!("{}", cls.get_data());
}

#[pyfunction]
fn convert_to_myclass(obj: &PyAny) -> PyResult<()> {
    match obj.extract::<MyClass>() {
        Ok(_) => println!("Converted to MyClass successfully"),
        Err(err) => println!("Could not convert object! {:?}", err),
    }
    Ok(())
}

#[pyfunction]
fn print_type_info(obj: &PyAny) {
    let typ = obj.get_type();
    println!("Object is of type: {}", typ);
    println!("isinstance(obj, MyClass): {}", typ.is_instance(obj).unwrap());
}

#[pymodule]
fn consumer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(print_data))?;
    m.add_wrapped(wrap_pyfunction!(print_type_info))?;
    m.add_wrapped(wrap_pyfunction!(convert_to_myclass))?;
    Ok(())
}
