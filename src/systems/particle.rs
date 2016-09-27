use cpython::{PyObject, PyResult};
use lumol;
use std::cell::RefCell;

register!(|py, m| {
    try!(m.add_class::<Particle>(py));
    Ok(())
});

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

#[cfg(test)]
mod tests {
    mod rust {
        use cpython::Python;
        use super::super::Particle;

        #[test]
        fn name() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));
            assert!(particle.name(py).unwrap() == "He");
            particle.set_name(py, "Kr").unwrap();
            assert!(particle.name(py).unwrap() == "Kr");
        }

        #[test]
        fn mass() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));
            assert!(particle.mass(py).unwrap() == 4.0026021003723145);
            particle.set_mass(py, 42.0).unwrap();
            assert!(particle.mass(py).unwrap() == 42.0);
        }

        #[test]
        fn charge() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));
            assert!(particle.charge(py).unwrap() == 0.0);
            particle.set_charge(py, 2.0).unwrap();
            assert!(particle.charge(py).unwrap() == 2.0);
        }
    }

    mod python {
        use cpython::Python;
        use super::super::Particle;

        #[test]
        fn name() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));

            py_run_with!(py, particle;
                "assert particle.name() == 'He'",
                "particle.set_name('Kr')",
                "assert particle.name() == 'Kr'"
            );
        }

        #[test]
        fn mass() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));

            py_run_with!(py, particle;
                "assert particle.mass() == 4.0026021003723145",
                "particle.set_mass(33)",
                "assert particle.mass() == 33"
            );
        }

        #[test]
        fn charge() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));

            py_run_with!(py, particle;
                "assert particle.charge() == 0.0",
                "particle.set_charge(2)",
                "assert particle.charge() == 2"
            );
        }
    }
}
