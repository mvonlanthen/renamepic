use std::fs;
use std::any::type_name;

extern crate clap;
use clap::{Arg, App};

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

    // let pictures_dir = matches.value_of("dir").unwrap_or("input.txt");
    let pictures_dir = matches.value_of("dir").unwrap();
    println!("The dir passed is: {}", pictures_dir);

    let x = 5;
    let y = 5.4;

    println!("{}",type_name::<i32>());
    println!("{}",type_name::<f32>());
}
