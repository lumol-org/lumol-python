#[macro_use]
extern crate cpython;
extern crate lumol;

mod systems;

py_module_initializer!(lumol, initlumol, PyInit_lumol, |py, m| {
    try!(m.add(py, "__doc__", "Modern and extensible molecular simulation engine"));

    try!(systems::particle::register(py, m));

    Ok(())
});
