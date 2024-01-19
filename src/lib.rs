#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Ensure the libraries are linked in, despite it not being used directly
#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src;
#[cfg(feature = "mumps")]
extern crate mumps_src;
