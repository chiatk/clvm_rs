use pyo3::prelude::{pyclass, pymethods};
use pyo3::types::PyString;
use pyo3::{PyAny, PyObject, PyResult, Python, ToPyObject};

use crate::allocator::Allocator;
use crate::cost::Cost;
use crate::reduction::Reduction;

use super::arena::Arena;
use super::error_bridge::raise_eval_error;
use super::f_table::OpFn;

#[pyclass]
pub struct NativeOp {
    pub op: OpFn,
}

impl NativeOp {
    pub fn new(op: OpFn) -> Self {
        Self { op }
    }
}

#[pymethods]
impl NativeOp {
    #[call]
    fn __call__<'p>(
        &'p self,
        py: Python<'p>,
        args: &'p PyAny,
        _max_cost: Cost,
    ) -> PyResult<(Cost, PyObject)> {
        let arena_cell = Arena::new_cell(py)?;
        let arena: &Arena = &arena_cell.borrow();
        let ptr = arena.ptr_for_obj(py, args)?;
        let mut allocator = arena.allocator();
        let allocator: &mut Allocator = &mut allocator;
        let r = (self.op)(allocator, ptr, _max_cost);
        match r {
            Ok(Reduction(cost, ptr)) => {
                let r = arena.py_for_native(py, ptr, allocator)?;
                Ok((cost, r.to_object(py)))
            }
            Err(_err) => {
                let r = arena.py_for_native(py, ptr, allocator)?;
                match raise_eval_error(
                    py,
                    PyString::new(py, "problem in suboperator"),
                    r.to_object(py),
                ) {
                    Err(e) => Err(e),
                    Ok(_) => panic!("oh dear"),
                }
            }
        }
    }
}
