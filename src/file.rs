use std::fs::DirEntry;
use fs_extra::dir::get_size;
use tabled::Tabled;
use crate::helpers::{ToPrettySize, SystemTimeStringHelpers};

#[derive(Tabled, Debug)]
pub struct File {
    pub name: String,
    pub size: String,
    pub size_in_bytes: u64,
    pub created_date: String,
    pub modified_date: String,
    pub extension: String,
}

impl File {
    pub fn from_dir_entry(entry: DirEntry) -> Self {
        let metadata = entry.metadata().unwrap();
        let size = match metadata.is_dir() {
            true => get_size(entry.path()).unwrap(),
            false => metadata.len()
        };
        let pretty_size = size.to_pretty_size();
        let name = entry.file_name().to_str().unwrap().to_string();

        let extension = match name.contains(".") {
            true => name.split(".").last().unwrap(),
            false => ""
        };

        Self {
            name: entry.file_name().to_str().unwrap().to_string(),
            size: pretty_size,
            size_in_bytes: size,
            created_date: metadata.created().unwrap().to_string(),
            modified_date: metadata.modified().unwrap().to_string(),
            extension: extension.to_string(),
        }
    }
}