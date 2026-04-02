fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++20")
        .include("cpp/include")
        .include("cpp/src")
        .file("cpp/src/xenor_sim.cpp")
        .file("cpp/src/xenor_state.cpp")
        .file("cpp/src/xenor_math.cpp")
        .file("cpp/src/xenor_checksum.cpp")
        .warnings(true)
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wpedantic")
        .compile("xenor_native_cpp");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cpp/include/xenor_types.h");
    println!("cargo:rerun-if-changed=cpp/include/xenor_sim.h");
    println!("cargo:rerun-if-changed=cpp/src/xenor_internal.hpp");
    println!("cargo:rerun-if-changed=cpp/src/xenor_sim.cpp");
    println!("cargo:rerun-if-changed=cpp/src/xenor_state.cpp");
    println!("cargo:rerun-if-changed=cpp/src/xenor_math.cpp");
    println!("cargo:rerun-if-changed=cpp/src/xenor_checksum.cpp");
}
