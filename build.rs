extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").expect("getting target");

    // --------- link to libdc1394 ---------
    enum LocationSource {
        EnvVar(String),
        PkgConfig,
    }

    let loc_source = match env::var("DC1394_LIBDIR") {
        Ok(dir) => LocationSource::EnvVar(dir),
        Err(_) => LocationSource::PkgConfig,
    };

    match loc_source {
        LocationSource::EnvVar(dir) => {
            println!("cargo:rustc-link-search=native={}",dir);
            println!("cargo:rustc-link-lib=static=dc1394");

            if target.contains("apple-darwin") {
                println!("cargo:rustc-link-lib=framework=CoreServices");
                println!("cargo:rustc-link-lib=framework=CoreFoundation");
                println!("cargo:rustc-link-lib=framework=IOKit");
            }

        },
        LocationSource::PkgConfig => {
            let result = pkg_config::Config::new()
                .atleast_version("2.1.2")
                .statik(true)
                .probe("libdc1394-2");
            match result {
                Ok(_) => {},
                Err(e) => {
                    panic!("No env var DC1394_LIBDIR could be read and pkg_config \
                           failed with {}", e);
                }
            }
        }
    }

    // --------- generate bindings for libdc1394 ---------
    let incdir = match env::var("DC1394_INCDIR") {
        Ok(dir) => Some(PathBuf::from(dir)),
        Err(_) => None,
    };

    let base = match &incdir {
        &Some(ref dir) => dir.clone(),
        &None => PathBuf::from("/usr/include"), // default
    };

    let header = base.join("dc1394").join("control.h");

    if !header.is_file() {
        panic!("no header found at {:?}. Set env var DC1394_INCDIR.", header);
    }

    let mut builder = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .constified_enum_module("dc1394.*_t");

    if let Some(p) = incdir {
        let arg = format!("-I{}",p.to_str().unwrap());
        builder = builder.clang_arg(arg);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
