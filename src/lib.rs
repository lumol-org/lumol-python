#[macro_use]
extern crate cpython;

py_module_initializer!(lumol, initlumol, PyInit_lumol, |py, m| {
    try!(m.add(py, "__doc__", "Modern and extensible molecular simulation engine"));
    Ok(())
});
