mod particle;
mod cell;

register!(|py, m| {
    try!(particle::register(py, m));
    try!(cell::register(py, m));
    Ok(())
});
