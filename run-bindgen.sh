#!/bin/bash
set -o errexit

bindgen $DC1394_INCDIR/dc1394/control.h \
    --distrust-clang-mangling \
    --constified-enum-module='dc1394.*_t' \
    --raw-line '#![allow(non_upper_case_globals)]' \
    --raw-line '#![allow(non_camel_case_types)]' \
    --raw-line '#![allow(non_snake_case)]' \
    --raw-line 'extern crate libusb_sys;' \
    -- -I$DC1394_INCDIR > src/lib.rs
