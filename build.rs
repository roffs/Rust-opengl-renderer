use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::fs::read_dir;

fn main() {
    println!("cargo:rerun-if-changed=res/*");

    let out_dir = env::var("OUT_DIR").unwrap();

    let copy_options = CopyOptions::new().overwrite(true);
    let mut paths_to_copy = Vec::new();

    let paths = read_dir("assets").unwrap();

    for path in paths {
        paths_to_copy.push(path.unwrap().path())
    }

    copy_items(&paths_to_copy, out_dir, &copy_options).unwrap();
}
