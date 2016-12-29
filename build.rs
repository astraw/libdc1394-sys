extern crate pkg_config;

fn main() {
    pkg_config::Config::new().atleast_version("2.2.4").probe("libdc1394-2").unwrap();
}
