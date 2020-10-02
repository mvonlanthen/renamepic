use std::fs;
use std::path;
use std::ffi;

extern crate clap;
use clap::{Arg, App};

// extern crate exif;
// use exif;

fn main() {
    println!("Hello, world!");

    let matches = App::new("renamepics")
        .version("0.1.0")
        .author("Marcel Vonlanthen")
        .about("rename pictures in a folder according their exif metadata. \
                Mostly used with exif datetime information, or GPS informations.")
        .arg(Arg::with_name("dir")
                 .short("d")
                 .long("dir")
                 .takes_value(true)
                 .required(true)
                 .help("Folder with pictures to rename"))
        .arg(Arg::with_name("list_exif")
                 .short("le")
                 .long("list-exif")
                 .takes_value(true)
                 .required(false)
                 .help("list available exifs for a given picture"))
        .get_matches();

    
    // Check if the folder passed as argument exists
    let pictures_dir = path::Path::new(matches.value_of("dir").unwrap());
    if pictures_dir.exists()==false {
        println!("`{}` is not a valid directory. exit", pictures_dir.display());
        std::process::exit(1)
    }

    // loop through each path and process if jpg
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
            let file = fs::File::open(&path).unwrap();
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader).unwrap();
            // for f in exif.fields() {
            //     println!("{} {} {}",
            //                 f.tag, f.ifd_num, f.display_value().with_unit(&exif));
            // }
            let datetime = exif.get_field(
                exif::Tag::DateTimeOriginal, 
                exif::In::PRIMARY
            ).unwrap();
            let dt_string = datetime.display_value().to_string();

            // println!("filename: {}, datetime: {}", 
            //     &path.display(), 
            //     dt_string
            // );

            // copy the jpg with it's new filename.
            let base_path = path.parent().unwrap().to_str().unwrap();
            let filename = path.file_name().and_then(ffi::OsStr::to_str).unwrap();
            let tgt_path = format!("{}/{} - {}", base_path, dt_string, filename);
            // let res = fs::copy(path, tgt_path);
            let is_successful = match fs::copy(&path, &tgt_path) {
                Ok(_nb_bytes) => true,
                Err(_error) => false,
            };
            if is_successful==false {
                println!(
                    "file {} cannot be copied to {}",
                    path.display(),
                    tgt_path,
                )
            }
        }

    }

}
