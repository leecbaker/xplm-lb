use std::env;
use std::path::PathBuf;
extern crate bindgen;

fn main() {
    // If we are running tests, then we almost certainly aren't running connected to
    // X-Plane where these APIs make sense.
    //
    // Instead, assume that all tests written are freestanding from the API.
    if !cfg!(test) {
        link_libraries();
    }

    let os_define = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "windows" => "IBM",
        "macos" => "APL",
        "linux" => "LIN",
        _ => panic!("Unsupported OS"),
    };

    let bindings_builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg("-I./SDK/CHeaders/XPLM")
        .clang_arg(format!("-D{os_define}"))
        .rustified_enum("XPLMAudioBus")
        .rustified_enum("XPLMBankID")
        .rustified_enum("XPLMCameraControlDuration")
        .rustified_enum("XPLMCommandPhase")
        .rustified_enum("XPLMCursorStatus")
        .rustified_enum("XPLMDataFileType")
        .rustified_enum("XPLMDeviceID")
        .rustified_enum("XPLMFlightLoopPhaseType")
        .rustified_enum("XPLMFontID")
        .rustified_enum("XPLMHostApplicationID")
        .rustified_enum("XPLMLanguageCode")
        .rustified_enum("XPLMMapLayerType")
        .rustified_enum("XPLMMapOrientation")
        .rustified_enum("XPLMMapStyle")
        .rustified_enum("XPLMMenuCheck")
        .rustified_enum("XPLMMouseStatus")
        .rustified_enum("XPLMNavType")
        .rustified_enum("XPLMWindowDecoration")
        .rustified_enum("XPLMWindowLayer")
        .rustified_enum("XPLMWindowPositioningMode")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Define which version of the SDK should be compiled. Only the major versions
    // are exposed as features. Version information is from the SDK download
    // page. <https://developer.x-plane.com/sdk/plugin-sdk-downloads/>
    let bindings_builder = if cfg!(feature = "xplm2") {
        bindings_builder
            .clang_arg("-DXPLM200") //X-Plane 9.00 & newer
            .clang_arg("-DXPLM210") //X-Plane 10.00 & newer (10.20 required for 64-bit plugins)
    } else {
        bindings_builder
    };

    let bindings_builder = if cfg!(feature = "xplm3") {
        bindings_builder
            .clang_arg("-DXPLM300") // X-Plane 11.10 & newer (64-bit only)
            .clang_arg("-DXPLM301") // X-Plane 11.20 & newer (64-bit only)
            .clang_arg("-DXPLM302") // No documentation
            .clang_arg("-DXPLM303") // X-Plane 11.50 & newer (64-bit only)
    } else {
        bindings_builder
    };

    let bindings_builder = if cfg!(feature = "xplm4") {
        bindings_builder
            .clang_arg("-DXPLM400") // X-Plane 12.04 & newer (64-bit only)
            .clang_arg("-DXPLM410") // Not documented
            .clang_arg("-DXPLM411") // X-Plane 12.1.0 & newer (64-bit only), but doesn't seem to affect any APIs
            .clang_arg("-DXPLM420") // X-Plane 12.3.0 and newer
    } else {
        bindings_builder
    };

    let bindings_builder = if cfg!(feature = "deprecated") {
        bindings_builder.clang_arg("-DXPLM_DEPRECATED")
    } else {
        bindings_builder
    };

    let bindings = bindings_builder
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

/// On Mac OS and Windows targets, links the XPLM libraries
fn link_libraries() {
    // Get the absolute path to this crate, so that linking will work when done in
    // another folder
    let crate_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let target = env::var("TARGET").unwrap();

    if target.contains("-apple-") {
        let library_path = crate_path.join("SDK/Libraries/Mac");
        println!(
            "cargo:rustc-link-search=framework={}",
            library_path.to_str().unwrap()
        );
        println!("cargo:rustc-link-lib=framework=XPLM");
    } else if target.contains("-linux-") {
        // Do nothing for Linux
    } else if target.contains("-windows-") {
        let library_path = crate_path.join("SDK/Libraries/Win");
        println!("cargo:rustc-link-search={}", library_path.to_str().unwrap());
        if target.contains("x86_64") {
            println!("cargo:rustc-link-lib=XPLM_64");
        } else {
            println!("cargo:rustc-link-lib=XPLM");
        }
    } else {
        panic!("Target operating system not Mac OS, Linux, or Windows")
    }
}
