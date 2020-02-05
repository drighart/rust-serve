use std::{env, fs};
use env_logger::{Builder,Env};
use log::{info,error};
use walkdir::WalkDir;

#[macro_use]
mod arguments;

use arguments::Arguments;


fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("I will help you to serve...");

    let args = Arguments::new().parse();

    info!("{:?}", args);
    read_folders2();
}

fn read_folders2() {
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {

        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata().unwrap().modified().unwrap();

        if f_name.ends_with(".json") && sec.elapsed().unwrap().as_secs() < 86400 {
            info!("{}", entry.path().display());
        }
    }
}

fn read_folders() {
    let current_dir = env::current_dir().unwrap();
    info!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            info!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename").unwrap()
            );
        }
    }
}