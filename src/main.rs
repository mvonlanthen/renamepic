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
        .get_matches();

    let myfile = matches.value_of("dir").unwrap_or("input.txt");
    println!("The dir passed is: {}", myfile);

    let x = 5;
    let y = 5.4;

    println!("{}",type_name::<i32>());
    println!("{}",type_name::<f32>());
}
