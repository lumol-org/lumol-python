use std::env;
use std::path::{Path, PathBuf};
use std::io::Result;
use std::fs;

#[cfg(unix)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    use std::os::unix::fs;
    return fs::symlink(src, dst);
}

#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    use std::os::windws::fs;
    return fs::symlink_file(src, dst);
}

fn dynlib_ext() -> &'static str {
    if cfg!(target_os="macos") {
        ".dylib"
    } else if cfg!(target_os="linux") {
        ".so"
    } else if cfg!(windows) {
        ".dll"
    } else {
        panic!("Missing dynamic library extension for this platform. Please edit 'build.rs'");
    }
}

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let libname = String::from("liblumol_py") + dynlib_ext();
    let library = PathBuf::from("target").join(profile).join(libname);
    let link = Path::new("./lumol.so");

    if link.symlink_metadata().is_ok() {
        fs::remove_file(link).expect("Could not remove symbolic link");
    }

    symlink(library, link).expect("Could not create symbolic link");
}
