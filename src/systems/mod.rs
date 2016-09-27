mod particle;

register!(|py, m| {
    try!(particle::register(py, m));
    Ok(())
});
