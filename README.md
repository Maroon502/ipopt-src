# Ipopt-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description

ipopt-src crate is a *-src crate. This links [Ipopt] libraries to executable build by cargo, but does not provide Rust bindings.

By this package, you don't need to worry about installing Ipopt in the system, and it's a package for **all platforms**.

[Ipopt] (Interior Point OPTimizer, pronounced eye-pea-Opt) is a software package for large-scale nonlinear optimization. It is designed to find (local) solutions of mathematical optimization problems of the NLP.

## Usage

1. add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    ipopt-src = "0.2"
    ```

2. add the following to your `lib.rs`:

    ```toml
    extern crate ipopt_src;
    ```

This package does not provide bindings. Please use [coinipopt-sys] to use Ipopt, e.g.

## Configuration

The following Cargo features are supported:

* `default` to enable `mumps` with `openblas-static`;

There's one of the following solvers needed by ipopt(click [Ipopt] to get more information):

* `intel-mkl` to build with Intel MKL;
* `mumps` to build with Mumps;
* `hsl` to build with HSL;(Not supported now)
* `spral` to build with Spral;(Not supported now)
* `wsmp` to build with WSMP;(Not supported now)

if `intel-mkl` is selected, you should choose one of the following linking mode:

* `mkl-static-lp64-seq` to build with Intel MKL;
* `mkl-dynamic-lp64-seq` to build with Intel MKL;

if `mumps` is selected, you should choose one of the following linking mode as mumps' liner solver:

* `mkl-static-lp64-seq` to build with Intel MKL;
* `mkl-dynamic-lp64-seq` to build with Intel MKL;
* `openblas-static` to build with OpenBLAS and link as static;
* `openblas-dynamic` to build with OpenBLAS and link as dynamic;

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

Because [openblas-src]'s Issue [#101](https://github.com/blas-lapack-rs/openblas-src/issues/101), we can't cross compile the package with `openblas-static` feature. So, if you want to cross compile the package, you could use [mike-kfed](https://github.com/mike-kfed/openblas-src/tree/arm-cross-compile) instead.

Add this to your `project/.cargo/config.toml`.

```toml
[patch.crates-io]
openblas-src = { git = "https://github.com/mike-kfed/openblas-src.git", branch = "arm-cross-compile" }
```

you can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-linux-androideabi`            | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`              | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| others                               | not test   |

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[Ipopt]: https://github.com/coin-or/Ipopt
[coinipopt-sys]: https://github.com/Maroon502/coinipopt-sys

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/ipopt-src/badge.svg
[documentation-url]: https://docs.rs/ipopt-src
[package-img]: https://img.shields.io/crates/v/ipopt-src.svg
[package-url]: https://crates.io/crates/ipopt-src
[license-img]: https://img.shields.io/crates/l/ipopt-src.svg
[license-url]: https://github.com/Maroon502/ipopt-src/blob/master/LICENSE.md
