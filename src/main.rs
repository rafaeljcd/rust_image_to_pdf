use std::env::{consts, current_dir};
use std::fs::{self};
use std::io::Error as IOError;

fn main() -> Result<(), IOError>{
    // print out the OS version
    let os = consts::OS;
    println!("{}", os);

    let cwd = match current_dir() {
        Ok(path_buf) => path_buf,
        Err(e) => panic!("error {}", e.to_string()),
    };

    println!("current directory: {}", cwd.to_string_lossy());

    // convert pathbuf to path
    let cwd_path = cwd.as_path();

    for entry in fs::read_dir(cwd_path)? {
        let entry_dir = entry?;
    }

    Ok(())
}
