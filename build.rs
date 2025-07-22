use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Build MicroTeX using cmake
    let dst = cmake::Config::new("ext/MicroTeX")
        .define("HAVE_LOG", "OFF")
        .define("GRAPHICS_DEBUG", "OFF")
        .build_target("LaTeX")
        .build();

    // 2. Compile our wrapper
    // Find cairo and cairomm includes
    let cairo_include = pkg_config::Config::new()
        .probe("cairo")
        .map(|lib| lib.include_paths)
        .unwrap_or_default();
    
    let cairomm_include = pkg_config::Config::new()
        .probe("cairomm-1.0")
        .map(|lib| lib.include_paths)
        .unwrap_or_default();
    
    let pangomm_include = pkg_config::Config::new()
        .probe("pangomm-1.4")
        .map(|lib| lib.include_paths)
        .unwrap_or_default();
    
    let mut build = cc::Build::new();
    build.cpp(true)
        .file("src/wrapper.cpp")
        .include("ext/MicroTeX/src")
        .include(&format!("{}/include", dst.display()))
        .flag("-std=c++17")
        .flag("-Wno-unused-parameter")  // Suppress unused parameter warnings from MicroTeX
        .flag("-Wno-reorder")          // Suppress initialization order warnings from MicroTeX
        .define("BUILD_GTK", None)      // Enable Cairo graphics backend
        .define("HAVE_LOG", None)       // Enable logging for debugging
        .object(&format!("{}/build/libLaTeX.a", dst.display())); // Link LaTeX lib directly
    
    // Add cairo include paths
    for path in cairo_include {
        build.include(path);
    }
    
    // Add cairomm include paths
    for path in cairomm_include {
        build.include(path);
    }
    
    // Add pangomm include paths
    for path in pangomm_include {
        build.include(path);
    }
    
    build.compile("microtex_wrapper");

    // 3. Link the compiled libraries
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=microtex_wrapper");
    println!("cargo:rustc-link-lib=static=LaTeX");
    // Link dependencies (matching MicroTeX's CMakeList.txt requirements)
    println!("cargo:rustc-link-lib=dylib=cairo");
    println!("cargo:rustc-link-lib=dylib=cairomm-1.0");
    println!("cargo:rustc-link-lib=dylib=pangomm-1.4");
    println!("cargo:rustc-link-lib=dylib=gtkmm-3.0");
    println!("cargo:rustc-link-lib=dylib=gtksourceviewmm-4.0");
    println!("cargo:rustc-link-lib=dylib=glibmm-2.4");
    println!("cargo:rustc-link-lib=dylib=sigc-2.0");
    println!("cargo:rustc-link-lib=dylib=fontconfig");
    println!("cargo:rustc-link-lib=dylib=tinyxml2");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    // 4. Generate bindings for our simplified wrapper
    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}