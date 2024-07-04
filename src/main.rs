mod args;
mod helpers;
mod file;

extern crate chrono;

use std::cmp::Ordering;
use std::fs::{canonicalize, read_dir};
use std::path::{PathBuf};
use std::time::SystemTime;
use tabled::{Table};
use tabled::settings::{Disable, Settings, Style};
use tabled::settings::location::ByColumnName;
use crate::args::{OrderBy, OrderDirection, parse_args};
use crate::helpers::{SystemTimeStringHelpers, ToPrettySize};
use crate::file::{File};

fn main() {
    let args = parse_args();

    let relative_path = PathBuf::from(args.path);
    let absolute_path = canonicalize(&relative_path).unwrap();
    let path = absolute_path.to_str().unwrap();

    let paths = read_dir(path).unwrap();
    let mut files: Vec<File> = vec![];

    for path in paths {
        let entry = path.unwrap();
        files.push(File::from_dir_entry(entry));
    }

    println!("Path: {}", if path.starts_with(r"\\?\") { &path[4..] } else { &path });
    println!("Ordering: {} ({})", args.order_by, args.order_direction.to_string().to_lowercase());

    let total_size: u64 = files.iter().map(|file| file.size_in_bytes).sum();
    println!("Total size: {}", total_size.to_pretty_size());

    files.sort_by(|a, b| {
        match args.order_by {
            OrderBy::Size => compare(&a.size_in_bytes, &b.size_in_bytes, &args.order_direction),
            OrderBy::Name => compare(&a.name, &b.name, &args.order_direction),
            OrderBy::CreatedDate => SystemTime::compare(&a.created_date.to_string(), &b.created_date.to_string(), &args.order_direction),
            OrderBy::ModifiedDate => SystemTime::compare(&a.modified_date, &b.modified_date, &args.order_direction),
            OrderBy::Extension => compare(&a.extension, &b.extension, &args.order_direction)
        }
    });

    fn compare<T: Ord>(a: T, b: T, direction: &OrderDirection) -> Ordering {
        match direction {
            OrderDirection::Ascending => { a.cmp(&b) }
            OrderDirection::Descending => { b.cmp(&a) }
        }
    }

    let table_config = Settings::default()
        .with(Style::modern_rounded())
        .with(Disable::column(ByColumnName::new("size_in_bytes")));
    let table = Table::new(files).with(table_config).to_string();
    println!("{}", table);
}


