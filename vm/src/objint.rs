use super::pyobject::{Executor, PyObject, PyObjectKind, PyObjectRef};
use std::collections::HashMap;

fn str(rt: &mut Executor, args: Vec<PyObjectRef>) -> Result<PyObjectRef, PyObjectRef> {
    Ok(rt.new_str("todo".to_string()))
}

fn add() {}

/*
fn set_attr(a: &mut PyObjectRef, name: String, b: PyObjectRef) {
    a.borrow().dict.insert(name, b);
}
*/

pub fn create_type(type_type: PyObjectRef) -> PyObjectRef {
    let mut dict = HashMap::new();
    dict.insert(
        "__str__".to_string(),
        PyObject::new(
            PyObjectKind::RustFunction { function: str },
            type_type.clone(),
        ),
    );
    let typ = PyObject::new(
        PyObjectKind::Class {
            name: "int".to_string(),
            // dict: PyObject::new(PyObjectKind::Dict { elements: dict }, type_type.clone() ),
        },
        type_type.clone(),
    );
    typ
}
