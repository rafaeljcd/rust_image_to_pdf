use std::env::{consts, current_dir};
fn main() {
    // print out the OS version
    let os = consts::OS;
    println!("{}", os);

    let cwd = current_dir()
        .expect("Error getting the current directory ");

    println!("current directory: {}", cwd.to_string_lossy());
}
