//! Archive
//! This module contains some archivers to `tar`, `tar.gz`

use crate::Configuration;
use chrono::{DateTime, Local};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn create_archive(config: &Configuration) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Calculate filename from datetime
    let now_date: DateTime<Local> = Local::now();
    let filename: String = format!("{}-{}.zip", config.zip_name, now_date);
    let path: String = format!("{}/{}", &config.cache_dir.display(), &filename);

    // Create cache dir if there is no directories
    std::fs::create_dir_all(&config.cache_dir)?;

    // Create Zip archive
    let file: File = File::create(&path)?;
    let mut zip = ZipWriter::new(&file);

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    add_file_recursive(&mut zip, &config.music_dir, &config.music_dir, options)?;
    zip.finish()?;

    Ok(path.into())
}

fn add_file_recursive(
    zip: &mut ZipWriter<&File>,
    root: &PathBuf,
    path: &PathBuf,
    options: SimpleFileOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            let p: String = format!("{}", entry.path().strip_prefix(root)?.display());

            zip.add_directory(p, options)?;
            add_file_recursive(zip, root, &entry.path(), options)?;
        } else {
            // Strip the filepath's prefix as `/home/haruki/Music`
            let p: String = format!("{}", entry.path().strip_prefix(root)?.display());

            let mut file: File = File::open(entry.path())?;
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf)?;

            zip.start_file(p, options)?;
            zip.write_all(&buf)?;
        }
    }

    Ok(())
}
