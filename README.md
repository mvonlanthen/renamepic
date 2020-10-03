# renamepic

Create copies of pictures in a directory. The copies are names 

    YYYY-MM-DD HH:MM:SS - orignal_name.original_ext

where `YYYY-MM-DD HH:MM:SS` is the DateTimeOriginal tag in the picture exif and `orignal_name.original_ext` the original name of the picture. When scanning the directory, only the following picture type are processed: jpg, JPG, jpeg and JPEG.

## installation

### from the release

### from the source
1. Download and install the rust toolchain available [here](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Go to the cloned repo and compile `renamepics` with  


## Notes
* This is my very first "real" Rust application. More a testing ground than anything else! Feel free to comment  (good or bad) the code.
