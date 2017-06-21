#![allow(unknown_lints, clippy)]

#[macro_use]
extern crate cpython;
extern crate lumol;

#[cfg(test)]
#[macro_use]
extern crate approx;

#[macro_use]
mod macros;
mod error;
mod traits;

mod systems;

py_module_initializer!(lumol, initlumol, PyInit_lumol, |py, m| {
    try!(m.add(py, "__doc__", "Modern and extensible molecular simulation engine"));
    try!(systems::register(py, m));
    Ok(())
});
