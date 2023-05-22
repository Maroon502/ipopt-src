#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "openblas")]
extern crate openblas_src;
#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src;