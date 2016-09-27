use cpython::{PyObject, PyErr, PyResult, PyTuple, ToPyObject, PythonObject};
use lumol;
use lumol::Vector3D;
use std::cell::RefCell;

use LumolError;

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

    def position(&self) -> PyResult<PyTuple> {
        let position = &self.particle(py).borrow().position;

        let x = position[0].to_py_object(py).into_object();
        let y = position[1].to_py_object(py).into_object();
        let z = position[2].to_py_object(py).into_object();

        Ok(PyTuple::new(py, &[x, y, z]))
    }

    def set_position(&self, position: &PyTuple) -> PyResult<PyObject> {
        if position.len(py) != 3 {
            return raise!(py, format!(
                "Wrong size for the position: should be a 3-dimmensional \
                tuple, but contains {} elements", position.len(py)
            ));
        }

        let x = try!(position.get_item(py, 0).extract::<f64>(py).or(
            raise!(py, "Position elements should be numbers"))
        );
        let y = try!(position.get_item(py, 1).extract::<f64>(py).or(
            raise!(py, "Position elements should be numbers"))
        );
        let z = try!(position.get_item(py, 2).extract::<f64>(py).or(
            raise!(py, "Position elements should be numbers"))
        );

        self.particle(py).borrow_mut().position = Vector3D::new(x, y, z);

        Ok(py.None())
    }

    def velocity(&self) -> PyResult<PyTuple> {
        let velocity = &self.particle(py).borrow().velocity;

        let x = velocity[0].to_py_object(py).into_object();
        let y = velocity[1].to_py_object(py).into_object();
        let z = velocity[2].to_py_object(py).into_object();

        Ok(PyTuple::new(py, &[x, y, z]))
    }

    def set_velocity(&self, velocity: &PyTuple) -> PyResult<PyObject> {
        if velocity.len(py) != 3 {
            return raise!(py, format!(
                "Wrong size for the velocity: should be a 3-dimmensional \
                tuple, but contains {} elements", velocity.len(py)
            ));
        }

        let x = try!(velocity.get_item(py, 0).extract::<f64>(py).or(
            raise!(py, "Velocity elements should be numbers"))
        );
        let y = try!(velocity.get_item(py, 1).extract::<f64>(py).or(
            raise!(py, "Velocity elements should be numbers"))
        );
        let z = try!(velocity.get_item(py, 2).extract::<f64>(py).or(
            raise!(py, "Velocity elements should be numbers"))
        );

        self.particle(py).borrow_mut().velocity = Vector3D::new(x, y, z);

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

        #[test]
        fn position() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));

            py_run_with!(py, particle;
                "assert particle.position() == (0.0, 0.0, 0.0)",
                "particle.set_position((1, 2, 3))",
                "assert particle.position() == (1, 2, 3)",
                "assert_raises(particle.set_position, (2, 3))",
                "assert_raises(particle.set_position, (1, 2, 3, 4))",
                "assert_raises(particle.set_position, ('1', 2, 3))",
                "assert_raises(particle.set_position, (1, '2', 3))",
                "assert_raises(particle.set_position, (1, 2, '3'))",
            );
        }

        #[test]
        fn velocity() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));

            py_run_with!(py, particle;
                "assert particle.velocity() == (0.0, 0.0, 0.0)",
                "particle.set_velocity((1, 2, 3))",
                "assert particle.velocity() == (1, 2, 3)",
                "assert_raises(particle.set_velocity, (2, 3))",
                "assert_raises(particle.set_velocity, (1, 2, 3, 4))",
                "assert_raises(particle.set_velocity, ('1', 2, 3))",
                "assert_raises(particle.set_velocity, (1, '2', 3))",
                "assert_raises(particle.set_velocity, (1, 2, '3'))",
            );
        }
    }
}
