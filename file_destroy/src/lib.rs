use std::fs::{remove_file, OpenOptions};
use std::io::{Write, Seek, SeekFrom};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug)]
pub enum DestroyError {
    QualityIsZero,
    FileNotFound(String),
    NoMetadata,
    DataNotWritten,
    UnableToRename,
    UnableToDelete,
}

impl std::error::Error for DestroyError {}
impl std::fmt::Display for DestroyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QualityIsZero => write!(f, "Quality should be more than zero."),
            Self::FileNotFound(path) => write!(f, "File '{}' is not found.", path),
            Self::NoMetadata => write!(f, "This file don't have metadata."),
            Self::DataNotWritten => write!(f, "This file can't be overwritten."),
            Self::UnableToRename => write!(f, "This file can't be renamed."),
            Self::UnableToDelete => write!(f, "This file can't be deleted.")
        }
    }
}

pub fn destroy(file_path: &str, quality: usize, rename: bool) -> Result<(), DestroyError> {
    if quality == 0 { return Err(DestroyError::QualityIsZero); }

    let mut file = match OpenOptions::new().write(true).open(file_path) {
        Ok(f) => Ok(f),
        Err(_) => Err(DestroyError::FileNotFound(file_path.to_string())),
    }?;
    let file_size = match file.metadata() {
        Ok(f) => Ok(f),
        Err(_) => Err(DestroyError::NoMetadata),
    }?.len();
    
    let pb = ProgressBar::new(quality as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    for _ in 0..quality {
        let data: Vec<u8> = (0..file_size).map(|_| rand::random::<u8>()).collect();
        let _ = file.seek(SeekFrom::Start(0));
        match file.write_all(&data) {
            Ok(_) => Ok(()),
            Err(e) => {println!("{}", e); Err(DestroyError::DataNotWritten)},
        }?;
        pb.inc(1);
    }

    let delete_path = if rename {
        let new_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let parts: Vec<&str> = file_path.split("/").collect();
        let n_parts = parts.len();
        let new_path = if n_parts > 1 {
            format!("{}/{}", parts[..n_parts-1].join("/"), new_name)
        } else {
            new_name
        };
        match std::fs::rename(file_path, &new_path) {
            Ok(_) => Ok(()),
            Err(_) => Err(DestroyError::UnableToRename),
        }?;
        new_path
    } else {
        file_path.to_string()
    };
    match remove_file(delete_path) {
        Ok(_) => Ok(()),
        Err(_) => Err(DestroyError::UnableToDelete)
    }?;

    Ok(())
}