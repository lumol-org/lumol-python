#[macro_use]
extern crate cpython;
extern crate lumol;

#[macro_use]
mod macros;
mod systems;

py_exception!(lumol, LumolError);

py_module_initializer!(lumol, initlumol, PyInit_lumol, |py, m| {
    try!(m.add(py, "__doc__", "Modern and extensible molecular simulation engine"));
    try!(systems::register(py, m));
    Ok(())
});
