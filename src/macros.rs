macro_rules! create_instance {
    ($py: ident, $Type: ty) => ({
        let class = $py.get_type::<$Type>();
        let instance: $Type = class.call($py, ::cpython::PyTuple::empty($py), None).unwrap().extract($py).unwrap();
        instance
    });
    ($py: ident, $Type: ty, $args: expr) => ({
        let class = $py.get_type::<$Type>();
        let instance: $Type = class.call($py, $args, None).unwrap().extract($py).unwrap();
        instance
    });
}

macro_rules! py_run_with {
    ($py: ident, $($obj: ident),+; $($code: expr),+ $(,)*) => ({
        const ADDITIONAL_DEFINITIONS: &'static str = "
def assert_raises(callable, *args, **kwargs):
    throw = True
    try:
        callable(*args, **kwargs)
        throw = False
    except LumolError:
        pass
    assert throw

def assert_approx_eq(a, b):
    # the abs function is not available when running from Rust
    abs = a - b
    if abs < 0:
        abs = -abs
    assert abs < 1e-12
";
        use cpython::PyDict;
        let locals = PyDict::new($py);
        $(
            locals.set_item($py, stringify!($obj), $obj).unwrap();
        )+

        let globals = PyDict::new($py);
        let error = $py.get_type::<$crate::error::LumolError>();
        globals.set_item($py, "LumolError", error).unwrap();

        py_run!($py, globals, locals, ADDITIONAL_DEFINITIONS);
        py_run!($py, globals, locals, $($code),+);
    });
}

macro_rules! py_run {
    ($py: ident, $globals: ident, $locals: ident, $code: expr) => ({
         $py.run($code, Some(&$globals), Some(&$locals)).expect($code);
    });
    ($py: ident, $globals: ident, $locals: ident, $code: expr, $($tail: expr),+) => ({
         $py.run($code, Some(&$globals), Some(&$locals)).expect($code);
         py_run!($py, $globals, $locals, $($tail),+);
    });
}

macro_rules! register {
    (|$py: ident, $m: ident| $closure: expr) => (
        pub fn register($py: ::cpython::Python, $m: &::cpython::PyModule) -> ::cpython::PyResult<()> {
            return $closure;
        }
    );
}

macro_rules! raise {
    ($py: ident, $args: expr) => ({
        Err(PyErr::new::<$crate::error::LumolError, _>($py, $args))
    });
}
