use cpython::{PyObject, PyErr, PyResult, PyTuple, ToPyObject, PythonObject};
use std::cell::RefCell;
use lumol;

use traits::Callback;

register!(|py, m| {
    try!(m.add_class::<Particle>(py));
    Ok(())
});

py_class!(class Particle |py| {
    data particle: Box<Callback<lumol::sys::Particle>>;
    def __new__(_cls, name: &str) -> PyResult<Particle> {
        Particle::create_instance(py,
            Box::new(RefCell::new(lumol::sys::Particle::new(name)))
        )
    }

    def name(&self) -> PyResult<String> {
        let mut name = String::new();
        self.particle(py).with_ref(&mut |atom| {
            name += atom.name();
        });
        Ok(name)
    }

    def set_name(&self, name: &str) -> PyResult<PyObject> {
        self.particle(py).with_mut(&mut |atom| {
            atom.set_name(name)
        });
        Ok(py.None())
    }

    def mass(&self) -> PyResult<f64> {
        let mut mass = 0.0;
        self.particle(py).with_ref(&mut |atom| {
            mass = atom.mass;
        });
        Ok(mass)
    }

    def set_mass(&self, mass: f64) -> PyResult<PyObject> {
        self.particle(py).with_mut(&mut |atom| {
            atom.mass = mass;
        });
        Ok(py.None())
    }

    def charge(&self) -> PyResult<f64> {
        let mut charge = 0.0;
        self.particle(py).with_ref(&mut |atom| {
            charge = atom.charge;
        });
        Ok(charge)
    }

    def set_charge(&self, charge: f64) -> PyResult<PyObject> {
        self.particle(py).with_mut(&mut |atom| {
            atom.charge = charge;
        });
        Ok(py.None())
    }

    def position(&self) -> PyResult<PyTuple> {
        let mut position = [0.0; 3];
        self.particle(py).with_ref(&mut |atom| {
            position[0] = atom.position[0];
            position[1] = atom.position[1];
            position[2] = atom.position[2];
        });
        Ok(PyTuple::new(py, &[
            position[0].to_py_object(py).into_object(),
            position[1].to_py_object(py).into_object(),
            position[2].to_py_object(py).into_object(),
        ]))
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

        self.particle(py).with_mut(&mut |atom| {
            atom.position[0] = x;
            atom.position[1] = y;
            atom.position[2] = z;
        });
        Ok(py.None())
    }

    def velocity(&self) -> PyResult<PyTuple> {
        let mut velocity = [0.0; 3];
        self.particle(py).with_ref(&mut |atom| {
            velocity[0] = atom.velocity[0];
            velocity[1] = atom.velocity[1];
            velocity[2] = atom.velocity[2];
        });
        Ok(PyTuple::new(py, &[
            velocity[0].to_py_object(py).into_object(),
            velocity[1].to_py_object(py).into_object(),
            velocity[2].to_py_object(py).into_object(),
        ]))
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

        self.particle(py).with_mut(&mut |atom| {
            atom.velocity[0] = x;
            atom.velocity[1] = y;
            atom.velocity[2] = z;
        });

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
            assert_eq!(particle.name(py).unwrap(), "He");
            particle.set_name(py, "Kr").unwrap();
            assert_eq!(particle.name(py).unwrap(), "Kr");
        }

        #[test]
        fn mass() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));
            assert_eq!(particle.mass(py).unwrap(), 4.002602);
            particle.set_mass(py, 42.0).unwrap();
            assert_eq!(particle.mass(py).unwrap(), 42.0);
        }

        #[test]
        fn charge() {
            let gil = Python::acquire_gil();
            let py = gil.python();
            let particle = create_instance!(py, Particle, ("He",));
            assert_eq!(particle.charge(py).unwrap(), 0.0);
            particle.set_charge(py, 2.0).unwrap();
            assert_eq!(particle.charge(py).unwrap(), 2.0);
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
                "assert particle.mass() == 4.002602",
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
