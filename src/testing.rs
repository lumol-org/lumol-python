macro_rules! create_instance {
    ($py: ident, $Type: ty, $args: expr) => ({
        let class = $py.get_type::<$Type>();
        let instance: $Type = class.call($py, $args, None).unwrap().extract($py).unwrap();
        instance
    });
}

macro_rules! py_run_with {
    ($py: ident, $obj: ident; $($code: expr),+) => ({
        use cpython::PyDict;
        let dict = PyDict::new($py);
        dict.set_item($py, stringify!($obj), $obj).unwrap();
        py_run!($py, dict, $($code),+);
    });
}

macro_rules! py_run {
    ($py: ident, $dict: ident, $code: expr) => ({
         $py.run($code, None, Some(&$dict)).expect($code);
    });
    ($py: ident, $dict: ident, $code: expr, $($tail: expr),+) => ({
         $py.run($code, None, Some(&$dict)).expect($code);
         py_run!($py, $dict, $($tail),+);
    });
}
