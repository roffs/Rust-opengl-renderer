use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::PathBuf;

fn main() {
    // Get current assets directory from the root
    let directory_to_copy = "assets";

    // Request the output directory
    let out = env::var("PROFILE").unwrap();
    let out = PathBuf::from(format!("target/{}", out));

    // Iterate over all files & directories in assets
    let paths_to_copy = vec![directory_to_copy];

    let copy_options = CopyOptions::new().overwrite(true);
    println!("OUT: {:?}", out);
    println!("PATHS_TO_COPY: {:?}", paths_to_copy);
    copy_items(&paths_to_copy, out, &copy_options).unwrap();
}
