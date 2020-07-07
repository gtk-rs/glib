use std::{ffi::OsString, fs, io, path::Path};

fn main() {
    if Path::new("hooks/").exists() {
        let _ = copy_hooks();
    }
}

fn copy_hooks() -> io::Result<()> {
    println!("cargo:rerun-if-changed=hooks/*");

    for dir_entry_res in fs::read_dir("hooks")? {
        let dir_entry = dir_entry_res?;
        let file_path = dbg!(dir_entry.path());
        let file_name = dir_entry.file_name();
        let mut dest = OsString::from(".git/hooks/");
        dest.push(file_name);
        fs::copy(file_path, dest)?;
    }

    Ok(())
}
