// use std::env;
// fn main() {
//     let caffe_dir = env::var("CAFFE_ROOT").unwrap();
// println!("cargo:rustc-link-search=native={}/.build_release/lib",
// caffe_dir);
// }

extern crate bindgen;

// use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;


// use bindgen::{Bindings, BindgenOptions, LinkType, Logger};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dir = Path::new(&out_dir).join("lib/");
    let dest_path = Path::new(&lib_dir).join("ffi.rs");

    let _ = fs::create_dir(&lib_dir);

    let mut bindings = bindgen::builder();
    bindings.forbid_unknown_types();

    // //environment specific
    let caffe_root = env::var("CAFFE_ROOT").unwrap();
    let caffe_header = Path::new(&caffe_root).join("include/caffe/ffi.hpp");
    let caffe_include = Path::new(&caffe_root).join("include/");
    let caffe_lib_dir = Path::new(&caffe_root).join(".build_release/lib/");

    bindings.link("caffe");
    bindings.match_pat("ffi.hpp");
    bindings.header(caffe_header.to_str().unwrap());
    bindings.clang_arg(format!("-I{} -DCPU_ONLY", caffe_include.to_str().unwrap()));

    let bindings = bindings.generate();
    let bindings = bindings.unwrap();
    bindings.write_to_file(&dest_path).unwrap();

    println!("cargo:include={}", dest_path.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", caffe_lib_dir.to_str().unwrap());
}
