# Ipopt-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description

`Ipopt-src` crate is a *-src crate. This links [Ipopt] libraries to executable build by cargo, but does not provide Rust bindings. [Ipopt] is built with [Mumps] ([mumps-src]) and [OpenBLAS] ([openblas-src])(Optional) and [Intel-MKL] ([intel-mkl-src])(Optional).

By this package, you don't need to worry about installing Ipopt in the system, and it's a package for **all platforms**.

[Ipopt] (Interior Point OPTimizer, pronounced eye-pea-Opt) is a software package for large-scale nonlinear optimization. It is designed to find (local) solutions of mathematical optimization problems of the NLP.

## Usage

1. add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    ipopt-src = "\*"
    ```

2. add the following to your `lib.rs`:

    ```toml
    extern crate ipopt_src;
    ```

This package does not provide bindings. Please use [coinipopt-sys] to use Ipopt, e.g.

```toml
[dependencies]
coinipopt-sys = { version = "\*" }
```

## Configuration

### Features

The following Cargo features are supported:

* `default` to enable `mumps` with `openblas-static`;

There's one of the following solvers needed by ipopt(click [Ipopt] to get more information):

* `intel-mkl` to build with Intel MKL;
* `mumps` to build with Mumps;
* `hsl` to build with HSL;(Not supported now)
* `spral` to build with Spral;(Not supported now)
* `wsmp` to build with WSMP;(Not supported now)

if `intel-mkl` is selected, you should choose one of the following linking mode:

* `intel-mkl-static` to build with Intel MKL `lp64`, `seq`;
* `intel-mkl-dynamic` to build with Intel MKL `lp64`, `seq`;

if `mumps` is selected, you should choose one of the following linking mode as mumps' liner solver:

* `intel-mkl-static` to build with Intel MKL `lp64`, `seq`;
* `intel-mkl-dynamic` to build with Intel MKL `lp64`, `seq`;
* `openblas-static` to build with OpenBLAS and link as static;
* `openblas-dynamic` to build with OpenBLAS and link as dynamic;

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_IPOPT_STATIC` to link to Ipopt statically;
* `CARGO_IPOPT_SYSTEM` to link to Ipopt system library;
* `CARGO_MUMPS_STATIC` to link to Mumps statically;
* `CARGO_MUMPS_SYSTEM` to link to Mumps system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

If you enable OpenBLAS([openblas-src]), you can also pass env to `make` by `OPENBLAS_*`. Read more at [here](#cross-compilation)

### Others

If you enable OpenBLAS([openblas-src]), you can link `OpenBLAS` staticaly or dynamicly by disable default feature and select what you like, for example:

```toml
ipopt-src = { version = "\*", default-features = no, features = ["mumps", "openblas-static"] }
```

Similarly, you can link Intel MKL ([intel-mkl-src]) with:

```toml
ipopt-src = { version = "\*", default-features = no, features = ["intel-mkl", "intel-mkl-system"] }
```

If you want more configuration, you can try this:

```toml
ipopt-src = { version = "\*", default-features = no, features = ["intel-mkl"] }
intel-mkl-src = { version = "\*", features = ["mkl-static-lp64-seq"] }
```

## Windows and vcpkg

On Windows, openblas need [vcpkg] to find Ipopt. Before building, you must have the correct Ipopt installed for your target triplet and kind of linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install  `ipopt` for the `x64-windows` triplet:

```sh
vcpkg install ipopt --triplet x64-windows
```

To link Ipopt statically, install `ipopt` for the `x64-windows-static-md` triplet:

```sh
vcpkg install ipopt --triplet x64-windows-static-md
```

To link Ipopt and C Runtime (CRT) statically, install `ipopt` for the `x64-windows-static` triplet:

```sh
vcpkg install ipopt --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

If you use OpenBLAS([openblas-src]), you need to set `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and `OPENBLAS_TARGET` to pass env to [OpenBLAS], ref:[openblas-src] and [OpenBLAS]. For example:

```sh
export OPENBLAS_TARGET=ARMV8
export OPENBLAS_HOSTCC=gcc
export OPENBLAS_CC=aarch64-linux-gnu-gcc
export OPENBLAS_FC=aarch64-linux-gnu-gfortran
```

you can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| others                               | not test   |

Note: Features `intel-mkl-*` can only be used for `x86_64-*`. Features `openblas-static` can only be used for `linux`.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[Ipopt]: https://github.com/coin-or/Ipopt
[mumps]: https://mumps-solver.org/
[OpenBLAS]: https://github.com/OpenMathLib/OpenBLAS
[intel-mkl]: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html

[mumps-src]: https://github.com/Maroon502/mumps-src
[openblas-src]: https://github.com/blas-lapack-rs/openblas-src
[intel-mkl-src]: https://github.com/rust-math/intel-mkl-src
[coinipopt-sys]: https://github.com/Maroon502/coinipopt-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/ipopt-src/badge.svg
[documentation-url]: https://docs.rs/ipopt-src
[package-img]: https://img.shields.io/crates/v/ipopt-src.svg
[package-url]: https://crates.io/crates/ipopt-src
[license-img]: https://img.shields.io/crates/l/ipopt-src.svg
[license-url]: https://github.com/Maroon502/ipopt-src/blob/master/LICENSE.md
