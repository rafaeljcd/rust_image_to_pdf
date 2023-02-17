use std::env::{consts, current_dir};
use std::io::Error as IOError;
use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with(".") || entry.depth() == 0)
        .unwrap_or(false)
}

fn main() -> Result<(), IOError> {
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

    for entry in WalkDir::new(cwd_path).into_iter().filter_entry(|e| {
        is_not_hidden(e)
            && (!e.file_name().to_string_lossy().starts_with("target"))
    }) {
        let entry_dir = entry?;

        println!("{}", entry_dir.into_path().to_string_lossy())
    }

    Ok(())
}
