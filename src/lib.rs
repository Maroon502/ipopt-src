#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "openblas")]
extern crate openblas-src;
#[cfg(feature = "intel-mkl")]
extern crate intel-mkl-src;