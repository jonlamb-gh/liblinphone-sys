use cmake::Config;
use std::path::Path;

fn main() {
    let dst = Config::new("vendor/linphone-sdk")
        .define("ENABLE_OPENH264", "ON")
        .define("ENABLE_WEBRTC_AEC", "OFF")
        .define("ENABLE_UNIT_TESTS", "OFF")
        .define("ENABLE_MKV", "OFF")
        .define("ENABLE_FFMPEG", "ON")
        .define("ENABLE_CXX_WRAPPER", "OFF")
        .define("ENABLE_NON_FREE_CODECS", "ON")
        .define("ENABLE_VCARD", "OFF")
        .define("ENABLE_BV16", "OFF")
        .define("ENABLE_V4L", "OFF")
        .build_target("all")
        .build();

    let out_path = Path::new(&dst);
    let sdk_path = out_path.join("build").join("linphone-sdk").join("desktop");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("c_types")
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}/include", sdk_path.display()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!(
        "cargo:rustc-link-search=native={}",
        sdk_path.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=linphone");
}
