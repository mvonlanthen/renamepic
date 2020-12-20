# renamepic

Rename all the pictures in a source directory from `orignal_name.org_ext` to 
`YYYYMMDD-HHMMSS_orignal_name.org_ext` where `YYYYMMDD-HHMMSS` is the 
DateTimeOriginal tag in the picture exif,`orignal_name` the picture original 
name and  `org_ext` the picture original extension. The renamed images are saved 
in the target directory given as input argument. 

A


## installation

### from the release

### from the source
1. Download and install the rust toolchain available [here](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Go to the cloned repo and compile `renamepics` with
    ```bash
    cargo build --release
    ```

## Building
For developer (and myself, otherwise I forget...). TODO:
* compilation:
    ```bash
    cargo build --release
    ```
* cross platform compilation:
* debug in Visual Studio Code:




## Notes
* This is my very first "real" Rust application. More a testing ground than 
anything else! Feel free to comment  (good or bad) the code.
