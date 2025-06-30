fn main() {
    let mut build = cxx_build::bridge("src/cxx_ffi.rs");
    build.file("ffi/src/tinybvh.cpp").std("c++20");

    let compiler = build.get_compiler();

    if compiler.is_like_msvc() {
        build.flag("/arch:AVX2");
    } else {
        build.flag("-mavx2");
        build.flag("-march=native");
    }

    build.compile("tinybvh");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=tinybvh/tiny_bvh.h");
    println!("cargo:rerun-if-changed=src/tiny_bvh.h");
    println!("cargo:rerun-if-changed=src/tiny_bvh.cpp");
}
