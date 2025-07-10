//! Archive
//! This module contains some archivers to `tar`, `tar.gz`

use chrono::{DateTime, Local};
use std::fs::File;
use std::path::Path;
use crate::Configuration;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn create_archive(config: &Configuration) -> Result<File, Box<dyn std::error::Error>> {
    // Calculate filename from datetime
    let now_date: DateTime<Local> = Local::now();
    let filename: String = format!("{}-{}.zip", config.zip_name, now_date);
    let path: String = format!("{}/{}", &config.cache_dir.display(), &filename);

    // Create cache dir if there is no directories
    std::fs::create_dir_all(&config.cache_dir)?;

    // Create Zip archive
    let file: File = File::create(&path)?;
    let mut zip = ZipWriter::new(&file);

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    add_file_recursive(&mut zip, &config.music_dir, options)?;
    zip.finish()?;
    
    Ok(file)
}

fn add_file_recursive<P>(zip: &mut ZipWriter<&File>, path: P, options: SimpleFileOptions) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    for entry in std::fs::read_dir(&path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            add_file_recursive(zip, entry.path(), options)?;
        } else {
            zip.start_file_from_path(&path, options)?;
        }
    }

    Ok(())
}
