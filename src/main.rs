extern crate rexif;

use std::fs;
use std::io;
use std::error::Error;

use rexif::ExifTag;

fn scan_entries() -> Result<(), io::Error> {
    for entry in fs::read_dir("2015-03-06-Gordon-Ramzay-Trianon")? {
        let dir = entry?;
        let pfname = dir.path();
        let fname = pfname.to_str();
        let file_name = fname.unwrap();

        match rexif::parse_file(&file_name) {
            Ok(exif) => {
                let entries: Vec<rexif::ExifEntry> = exif.entries.into_iter().filter(|en| 
                    en.tag == ExifTag::GPSLatitudeRef ||
                    en.tag == ExifTag::GPSLatitude ||
                    en.tag == ExifTag::GPSLongitudeRef ||
                    en.tag == ExifTag::GPSLongitude).collect();
                println!("{:?}", dir.path());
                for entry in entries {
                            println!("	{}: {}",
                            entry.tag, 
                            entry.value);
                }
            },
            Err(e) => {
                print!("Error in {}: {}", &file_name,
                    Error::description(&e));
            }
        }
    }
    Ok(())
}

fn main() {
    match scan_entries() {
        Ok(()) => {}
        Err(e) => panic!("Woop {}", e)
    }
}