use cpython::{PyObject, PyResult, ToPyObject, PythonObject};
use std::cell::RefCell;
use lumol;

use cpython::py_class::CompareOp;

use traits::Callback;

register!(|py, m| {
    try!(m.add_class::<CellShape>(py));
    try!(m.add_class::<UnitCell>(py));
    Ok(())
});

py_class!(class CellShape |py| {
    data shape: lumol::sys::CellShape;

    @classmethod def triclinic(_cls) -> PyResult<CellShape> {
        CellShape::create_instance(py, lumol::sys::CellShape::Triclinic)
    }

    @classmethod def orthorhombic(_cls) -> PyResult<CellShape> {
        CellShape::create_instance(py, lumol::sys::CellShape::Orthorhombic)
    }

    @classmethod def infinite(_cls) -> PyResult<CellShape> {
        CellShape::create_instance(py, lumol::sys::CellShape::Infinite)
    }

    def __repr__(&self) -> PyResult<String> {
        let repr = match *self.shape(py) {
            lumol::sys::CellShape::Infinite => "CellShape.Infinite",
            lumol::sys::CellShape::Orthorhombic => "CellShape.Orthorhombic",
            lumol::sys::CellShape::Triclinic => "CellShape.Triclinic",
        };
        Ok(repr.into())
    }

    def __richcmp__(&self, other: &PyObject, op: CompareOp) -> PyResult<PyObject> {
        let other = match other.extract::<CellShape>(py) {
            Ok(other) => other,
            Err(_) => return Ok(py.NotImplemented())
        };
        match op {
            CompareOp::Eq => {
                let result = self.shape(py) == other.shape(py);
                Ok(result.to_py_object(py).into_object())
            }
            CompareOp::Ne => {
                let result = self.shape(py) != other.shape(py);
                Ok(result.to_py_object(py).into_object())
            }
            _ => Ok(py.NotImplemented())
        }
    }
});

py_class!(class UnitCell |py| {
    data cell: Box<Callback<lumol::sys::UnitCell>>;
    def __new__(_cls,
                a: f64 = 0.0, b: f64 = a, c: f64 = a,
                alpha: f64=90.0, beta: f64=90.0, gamma: f64=90.0) -> PyResult<UnitCell> {
        let cell = if a == 0.0 && b == 0.0 && c == 0.0 {
            lumol::sys::UnitCell::new()
        } else if alpha == 90.0 && alpha == 90.0 && alpha == 90.0 {
            lumol::sys::UnitCell::ortho(a, b, c)
        } else {
            lumol::sys::UnitCell::triclinic(a, b, c, alpha, beta, gamma)
        };
        UnitCell::create_instance(py, Box::new(RefCell::new(cell)))
    }

    def a(&self) -> PyResult<f64> {
        let mut a = 0.0;
        self.cell(py).with_ref(&mut |cell| a = cell.a());
        Ok(a)
    }

    def b(&self) -> PyResult<f64> {
        let mut b = 0.0;
        self.cell(py).with_ref(&mut |cell| b = cell.b());
        Ok(b)
    }

    def c(&self) -> PyResult<f64> {
        let mut c = 0.0;
        self.cell(py).with_ref(&mut |cell| c = cell.c());
        Ok(c)
    }

    def alpha(&self) -> PyResult<f64> {
        let mut alpha = 0.0;
        self.cell(py).with_ref(&mut |cell| alpha = cell.alpha());
        Ok(alpha)
    }

    def beta(&self) -> PyResult<f64> {
        let mut beta = 0.0;
        self.cell(py).with_ref(&mut |cell| beta = cell.beta());
        Ok(beta)
    }

    def gamma(&self) -> PyResult<f64> {
        let mut gamma = 0.0;
        self.cell(py).with_ref(&mut |cell| gamma = cell.gamma());
        Ok(gamma)
    }

    def shape(&self) -> PyResult<CellShape> {
        let mut shape = lumol::sys::CellShape::Infinite;
        self.cell(py).with_ref(&mut |cell| shape = cell.shape());
        CellShape::create_instance(py, shape)
    }
});

#[cfg(test)]
mod tests {
    mod rust {
        use cpython::Python;
        use super::super::UnitCell;

        #[test]
        fn constructors() {
            let gil = Python::acquire_gil();
            let py = gil.python();

            let cell = create_instance!(py, UnitCell);
            assert_ulps_eq!(cell.a(py).unwrap(), 0.0);
            assert_ulps_eq!(cell.b(py).unwrap(), 0.0);
            assert_ulps_eq!(cell.c(py).unwrap(), 0.0);
            assert_ulps_eq!(cell.alpha(py).unwrap(), 90.0);
            assert_ulps_eq!(cell.beta(py).unwrap(), 90.0);
            assert_ulps_eq!(cell.gamma(py).unwrap(), 90.0);

            let cell = create_instance!(py, UnitCell, (3.0, 4.0, 5.0));
            assert_ulps_eq!(cell.a(py).unwrap(), 3.0);
            assert_ulps_eq!(cell.b(py).unwrap(), 4.0);
            assert_ulps_eq!(cell.c(py).unwrap(), 5.0);
            assert_ulps_eq!(cell.alpha(py).unwrap(), 90.0);
            assert_ulps_eq!(cell.beta(py).unwrap(), 90.0);
            assert_ulps_eq!(cell.gamma(py).unwrap(), 90.0);

            let cell = create_instance!(py, UnitCell, (3.0, 4.0, 5.0, 80.0, 90.0, 100.0));
            assert_ulps_eq!(cell.a(py).unwrap(), 3.0);
            assert_ulps_eq!(cell.b(py).unwrap(), 4.0);
            assert_ulps_eq!(cell.c(py).unwrap(), 5.0);
            assert_ulps_eq!(cell.alpha(py).unwrap(), 80.0);
            assert_ulps_eq!(cell.beta(py).unwrap(), 90.0);
            assert_ulps_eq!(cell.gamma(py).unwrap(), 100.0);
        }
    }

    mod python {
        use cpython::Python;
        use super::super::{UnitCell, CellShape};

        #[test]
        fn constructors() {
            #![allow(non_snake_case)]

            let gil = Python::acquire_gil();
            let py = gil.python();
            let UnitCell = py.get_type::<UnitCell>();
            let CellShape = py.get_type::<CellShape>();

            py_run_with!(py, UnitCell, CellShape;
                "cell = UnitCell()",
                "assert cell.a() == 0.0",
                "assert cell.b() == 0.0",
                "assert cell.c() == 0.0",
                "assert cell.alpha() == 90.0",
                "assert cell.beta() == 90.0",
                "assert cell.gamma() == 90.0",
                "assert cell.shape() == CellShape.infinite()",
            );

            let UnitCell = py.get_type::<UnitCell>();
            let CellShape = py.get_type::<CellShape>();

            py_run_with!(py, UnitCell, CellShape;
                "cell = UnitCell(3, 4, 5)",
                "assert cell.a() == 3.0",
                "assert cell.b() == 4.0",
                "assert cell.c() == 5.0",
                "assert cell.alpha() == 90.0",
                "assert cell.beta() == 90.0",
                "assert cell.gamma() == 90.0",
                "assert cell.shape() == CellShape.orthorhombic()",
            );

            let UnitCell = py.get_type::<UnitCell>();
            let CellShape = py.get_type::<CellShape>();

            py_run_with!(py, UnitCell, CellShape;
                "cell = UnitCell(3, 4, 5, 80, 90, 100)",
                "assert_approx_eq(cell.a(), 3.0)",
                "assert_approx_eq(cell.b(), 4.0)",
                "assert_approx_eq(cell.c(), 5.0)",
                "assert_approx_eq(cell.alpha(), 80.0)",
                "assert_approx_eq(cell.beta(), 90.0)",
                "assert_approx_eq(cell.gamma(), 100.0)",
                "assert cell.shape() == CellShape.triclinic()",
            );
        }
    }
}
