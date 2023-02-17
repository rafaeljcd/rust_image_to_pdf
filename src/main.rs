use std::env::{consts, current_dir};
use std::fs::read_dir;
fn main() {
    // print out the OS version
    let os = consts::OS;
    println!("{}", os);

    let cwd = current_dir().expect("Error getting the current directory ");

    println!("current directory: {}", cwd.to_string_lossy());

    // convert pathbuf to path
    let cwd_path = cwd.as_path();

    for entry in read_dir(cwd_path)? {
        let dirEntry = entry.expect("Error directory");

        println!("entry {}", dirEntry.file_name().to_string_lossy());
    }
}
