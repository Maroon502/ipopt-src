use std::env;
use std::path::{Path, PathBuf};
use std::io::Write;

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "Ipopt";

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

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["IPOPT".to_string()];

        let link_type = if utils::want_static(LIB_NAME) {
            "static".to_string()
        } else {
            "dylib".to_string()
        };

        coinbuilder::print_metadata(Vec::new(), coinflags);
        return;
    }

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
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpPardisoMKLSolverInterface.cpp", src_dir));
        coinflags.push("PARDISO_MKL".to_string());
    }

    if cfg!(feature = "openblas") {
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMc19TSymScalingMethod.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMa27TSolverInterface.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMa57TSolverInterface.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMa77SolverInterface.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMa86SolverInterface.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpMa97SolverInterface.cpp", src_dir));
        lib_sources.push(format!("{}/Algorithm/LinearSolvers/IpPardisoSolverInterface.cpp", src_dir));

        coinflags.push("LAPACK".to_string());
    }

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("IPOPT_HAS_{}", flag), None);
    });
    config.define("IPOPT_HAS_RAND", None)
        .define("IPOPT_HAS_STD__RAND", None);
    config.define("IPOPTLIB_BUILD", None);

    // give a config.h file to the compiler
    if target.contains("linux") {
        let path = src_dir.clone() + "/config.h";
        let mut file = std::fs::File::create(path).unwrap();
        file.flush();

        config
            .define("IPOPT_LAPACK_FUNC(name,NAME)", Some("name ## _"))
            .define("IPOPT_LAPACK_FUNC_(name,NAME)", Some("name ## _"))
            .define("IPOPT_C_FINITE", Some("std::isfinite"))
            .define("HAVE_CONFIG_H", None)
            .include(&src_dir);
    }

    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);
}
