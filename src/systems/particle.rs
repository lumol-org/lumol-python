use cpython::{PyObject, PyResult, PyModule, Python};
use lumol;
use std::cell::RefCell;

pub fn register(py: Python, m: &PyModule) -> PyResult<()> {
    try!(m.add_class::<Particle>(py));
    Ok(())
}


py_class!(class Particle |py| {
    data particle: RefCell<lumol::Particle>;
    def __new__(_cls, name: &str) -> PyResult<Particle> {
        Particle::create_instance(py, RefCell::new(lumol::Particle::new(name)))
    }

    def name(&self) -> PyResult<String> {
        Ok(self.particle(py).borrow().name().into())
    }

    def set_name(&self, name: &str) -> PyResult<PyObject> {
        self.particle(py).borrow_mut().set_name(name);
        Ok(py.None())
    }

    def mass(&self) -> PyResult<f64> {
        Ok(self.particle(py).borrow().mass)
    }

    def set_mass(&self, mass: f64) -> PyResult<PyObject> {
        self.particle(py).borrow_mut().mass = mass;
        Ok(py.None())
    }

    def charge(&self) -> PyResult<f64> {
        Ok(self.particle(py).borrow().charge)
    }

    def set_charge(&self, charge: f64) -> PyResult<PyObject> {
        self.particle(py).borrow_mut().charge = charge;
        Ok(py.None())
    }
});
