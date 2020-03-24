// Build script for linking to ca

use std::env;

fn main()
{
    let lib_path = env::var("EPICS_LIB_PATH")
        .expect("Must define EPICS_LIB_PATH");
    println!("cargo:rustc-link-search={:}", lib_path);
}
