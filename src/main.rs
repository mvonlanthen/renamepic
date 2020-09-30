use std::fs;
// use std::any::type_name;
use std::path;

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

    
    // Check if the folder passed as argument exists
    // let pictures_dir = matches.value_of("dir").unwrap();
    let pictures_dir = path::Path::new(matches.value_of("dir").unwrap());
    println!("The dir passed is `{}`", pictures_dir.display());
    if pictures_dir.exists()==true {
        println!("`{}` exists", pictures_dir.display());
    } else {
        println!("`{}` is not a valid directory. exit", pictures_dir.display());
        std::process::exit(1)
    }

    // list all the jpg in picture
    let paths = fs::read_dir(pictures_dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension();
        let filename = path.file_name();
        println!(
            "Path: {}, filename {}", 
            path.display(),
            filename.ok_or("No filename").unwrap().to_str().unwrap()
            // extension.ok_or("No extension").unwrap()
        );
        // let foo = filename.ok_or("No filename").unwrap().to_str().unwrap();
        // println!("Name: {:?}", filename.ok_or("No filename").unwrap());
    }
        




    let x:f32 = 5.0;
    println!("the value of x is: {}", x);
    let y = &x;
    let z = y+x;

    // println!("{}",type_name::<i32>());
    // println!("{}",type_name::<f32>());
}
