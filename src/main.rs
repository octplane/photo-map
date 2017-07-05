extern crate rexif;
extern crate rreverse;

use std::fs;
use std::io;
use std::error::Error;
use std::collections::HashMap;

use rexif::ExifTag;
use rexif::TagValue;

use rreverse::Locations;

struct PhotoInContext {
}

#[derive(Debug)]
enum CoordOrValue {
    Coordinate(f64),
    Orientation(String),
}

fn scan_entries() -> Result<Vec<PhotoInContext>, io::Error> {
    let  photos = Vec::new();
    for entry in fs::read_dir("./sample")? {
         if let Ok(entry) = entry {
            // Here, `entry` is a `DirEntry`.
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    
                } else if file_type.is_file() {
                    let pfname = entry.path();
                    let fname = pfname.to_str();
                    let file_name = fname.unwrap();
                    println!("File: {}", file_name);

                    match rexif::parse_file(&file_name) {
                        Ok(exif) => {
                            let entries: HashMap<rexif::ExifTag, CoordOrValue> = exif.entries.into_iter().filter(|en| 
                                en.tag == ExifTag::GPSLatitudeRef ||
                                en.tag == ExifTag::GPSLatitude ||
                                en.tag == ExifTag::GPSLongitudeRef ||
                                en.tag == ExifTag::GPSLongitude).map(|entry| {
                                let tuple: (rexif::ExifTag, CoordOrValue) = match entry.value {
                                    TagValue::URational(s) => {
                                        let float_coordinates = s[0].value() + s[1].value()/60.0f64 + s[2].value()/3600.0f64;
                                        (entry.tag, CoordOrValue::Coordinate(float_coordinates))
                                    }
                                    _ => {
                                        let string_value = format!("{}", entry.value);
                                        (entry.tag, CoordOrValue::Orientation(string_value))
                                    }
                                };
                                tuple
                            }).collect();
                            println!("{:?}", entries);
                        },
                        Err(e) => {
                            print!("Error in {}: {}", &file_name,
                                Error::description(&e));
                        }
                    }
                }
            }
        }
    }
    Ok(photos)
}

fn main() {
    match scan_entries() {
        Ok(_) => {}
        Err(e) => panic!("Woop {}", e)
    }
}