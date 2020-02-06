use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;
use log::{info,error};
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Cache {
    value: String,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            value: "localhost".to_string(),
        }
    }

    fn read_file(&self, filename: &str) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();

        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(why) => {
                error!("couldn't open {}", filename);
                return Err(why);
            },
        };

        if let Err(err) = file.read_to_end(&mut buf) {
            return Err(err);
        }

        return Ok(buf);
    }

    pub fn read_from_directory(&self, folder: &str) -> Cache {
        let cache = Cache::new();

        for entry in WalkDir::new(folder)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok()) {

            if !entry.path().is_dir() {
                let filename = entry.path().display().to_string();
                info!("Read file: {}", filename);

                let buf = self.read_file(&filename);
            }
        }

        return cache;
    }

}
