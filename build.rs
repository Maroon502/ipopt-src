use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, utils};

const LIB_NAME: &str = "Ipopt";
const IPOPT_VERSION: &str = "3.14.14";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join("src")
            .display()
    );
    let out_dir = env::var("OUT_DIR").unwrap();

    let target = env::var("TARGET").unwrap();

    let mut includes_dir = vec![
        format!("{}/Common", src_dir),
        format!("{}/Interfaces", src_dir),
        format!("{}/Algorithm", src_dir),
        format!("{}/Algorithm/LinearSolvers", src_dir),
        format!("{}/LinAlg", src_dir),
        format!("{}/LinAlg/TMatrices", src_dir),
        format!("{}/contrib/CGPenalty", src_dir),
        format!("{}/Apps/AmplSolver", src_dir),
    ];

    let mut lib_sources = include_str!("ipopt_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["IPOPT".to_string()];

    if cfg!(feature = "intel-mkl") {
        if !target.contains("x86_64") {
            panic!("Intel MKL is only supported on x86_64");
        }
        lib_sources.push(format!(
            "{}/Algorithm/LinearSolvers/IpPardisoMKLSolverInterface.cpp",
            src_dir
        ));
        coinflags.push("PARDISO_MKL".to_string());
        coinflags.push("LAPACK".to_string());
    }

    if cfg!(feature = "mumps") {
        let (include, _) = coinbuilder::get_metadata_from("Mumps");
        includes_dir.extend(include);
        lib_sources.push(format!(
            "{}/Algorithm/LinearSolvers/IpMumpsSolverInterface.cpp",
            src_dir
        ));
        coinflags.push("LAPACK".to_string());
        coinflags.push("MUMPS".to_string());
        coinflags.push("FEENABLEEXCEPT".to_string());
        coinflags.push("MPIINIT".to_string());

    }

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("IPOPT_HAS_{}", flag), None);
    });
    config
        .define("IPOPT_HAS_RAND", None)
        .define("IPOPT_HAS_STD__RAND", None);
    config.define("IPOPTLIB_BUILD", None).define(
        "IPOPT_VERSION",
        Some(format!("\"{}\"", IPOPT_VERSION).as_str()),
    );

    // give a config.h file to the compiler
    if target.contains("linux") {
        let path = out_dir.clone() + "/config.h";
        let mut file = std::fs::File::create(path).unwrap();
        file.flush()
            .expect("IO ERROR, Please clean and rebuild it!");

        config
            .define("IPOPT_LAPACK_FUNC(name,NAME)", Some("name ## _"))
            .define("IPOPT_LAPACK_FUNC_(name,NAME)", Some("name ## _"))
            .define("IPOPT_C_FINITE", Some("std::isfinite"))
            .define("HAVE_CONFIG_H", None)
            .include(&out_dir);
    }

    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);
}
