extern crate pkg_config;

use std::env;

fn main() {
    let target = env::var("TARGET").expect("getting target");

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

            // should we simply depend on libusb-sys?
            println!("cargo:rustc-link-lib=static=usb-1.0");

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

}
