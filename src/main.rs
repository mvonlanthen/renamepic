use std::fs;
use std::path;
use std::ffi;

extern crate clap;
use clap::{Arg, App};

extern crate chrono;
// use chrono::{NaiveDateTime};
use chrono::prelude::*;
extern crate exif;
// use exif;

fn main() {
    let matches = App::new("renamepics")
        .version("0.1.0")
        .author("Marcel Vonlanthen")
        .about("rename pictures in a folder according their exif metadata. \
                Mostly used with exif datetime information, or GPS informations.")
        .arg(Arg::with_name("source_dir")
                 .short("s")
                 .long("source-dir")
                 .takes_value(true)
                 .required(true)
                 .help("Source directory with pictures to rename"))
                 .arg(Arg::with_name("target_dir")
                 .short("t")
                 .long("taget-dir")
                 .takes_value(true)
                 .required(true)
                 .help("traget directory where the renamed images will go"))
        .arg(Arg::with_name("middle_string")
                 .short("m")
                 .long("middle-string")
                 .takes_value(true)
                 .required(false)
                 .help("add a middle string between the exif datetime and the picture filename"))
        .arg(Arg::with_name("list_exif")
                 .short("l")
                 .long("list-exif")
                 .takes_value(true)
                 .required(false)
                 .help("list available exifs for a given picture pass after `le` \
                        flag (not implemented yet)"))
        
        .get_matches();

    
    // Check if the folder passed as argument exists
    // let foo = matches.value_of("target_dir").unwrap();
    let pictures_dir = path::Path::new(matches.value_of("source_dir").unwrap());
    if pictures_dir.exists()==false {
        println!("`{}` is not a valid directory. exit", pictures_dir.display());
        std::process::exit(1)
    }

    // list all item in `dir` and loop through each item (`paths`) and process if jpg
    let paths = fs::read_dir(pictures_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let jpg_exts = vec!["jpg", "jpeg", "JPG", "JPEG"];
        let is_jpg = match path.extension() {
            None => false,
            Some(os_str) => {
                jpg_exts.iter().any(|&i| i==os_str)
            }
        };

        if is_jpg {
            // get DateTimeOriginal exif tag from the jpg 
            let file = fs::File::open(&path).unwrap();
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader).unwrap();
            let datetime = exif.get_field(
                exif::Tag::DateTimeOriginal, 
                exif::In::PRIMARY
            ).unwrap();
            // let dt_string = datetime.display_value().to_string();

            // parse the exif datetime with chrono and convert it into a string
            let datetime = NaiveDateTime::parse_from_str(
                &datetime.display_value().to_string(), "%Y-%m-%d %H:%M:%S"
            ).unwrap();
            let dt_string = format!("{:02}{:02}{:02}-{:02}{:02}{:02}",
                datetime.year(), datetime.month(), datetime.day(),
                datetime.hour(), datetime.minute(), datetime.second());

            // Prepare the target name path for the jpeg
            // let base_path = path.parent().unwrap().to_str().unwrap();
            let filename = path.file_name().and_then(ffi::OsStr::to_str).unwrap();
            let middle_string = match matches.value_of("middle_string") {
                Some(s) => format!("_{}", s),
                None => String::from(""),
            };
            let mut tgt_path = path::PathBuf::new();
            tgt_path.push(matches.value_of("target_dir").unwrap());
            tgt_path.push(format!("{}{}_{}", dt_string, middle_string, filename));

            // copy the picture to its new location
            let _is_successful = match fs::copy(&path, &tgt_path) {
                Ok(_nb_bytes) => true,
                Err(error) =>{
                    println!(
                        "file {} cannot be copied to {}. error: {}", 
                        path.display(), tgt_path.display(), error
                    );
                    false
                },
            };
        }

    }

}
