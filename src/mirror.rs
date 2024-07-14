use std::{
    fs::{set_permissions, OpenOptions, Permissions},
    io::Write,
    os::unix::prelude::PermissionsExt,
    process::Command,
};

use crate::config::Config;

pub fn copy_binary() {
    let config = Config::new();
    let config_folder_path = config.config_folder_path.clone();
    let out_path = config_folder_path.join("mirrorer");
    let bytes = include_bytes!("MirrorMacBookToMonitor");
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(out_path.clone())
        .expect("Failed to create or open file");
    output_file
        .write_all(bytes)
        .expect("Failed to copy mirrorer");
    set_permissions(out_path, Permissions::from_mode(0o755)).expect("Error setting permissions");
}

pub fn execute() {
    let config = Config::new();
    let config_folder_path = config.config_folder_path.clone();
    let out_path = config_folder_path.join("mirrorer");
    match Command::new(out_path).output() {
        Err(e) => println!("Error: {}", e),
        Ok(_) => println!("Executed"),
    }
}
