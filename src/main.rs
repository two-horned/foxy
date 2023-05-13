use std::env::var_os;
use std::fs::{File, DirBuilder, read_to_string};
use std::ffi::OsStr;
use std::io;

fn main() -> io::Result<()> {
    // Check if user is not root
    if var_os("USER").expect("Can't identfiy user") == "root" {
        panic!("User mustn't be root");
    }

    let mut c = var_os("HOME")
        .expect("Can't identify home");
    let t = var_os("XDG_CONFIG_HOME");
    match t {
        None => c.push(OsStr::new("/.config")),
        _ => c = t.unwrap(),
    };

    DirBuilder::new()
        .recursive(true)
        .create(&c)?;
    c.push(OsStr::new("/foxy.conf"));

    let content = read_to_string(c);
    println!("Content:\n{:?}", content);
    Ok(())
}
