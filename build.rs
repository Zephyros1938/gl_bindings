use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=gl.h");
    println!("cargo:rerun-if-changed=glfw.h");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=glfw");

    let gl_header = env::var("GL_HEADER").unwrap_or("gl.h".into());
    let glfw_header = env::var("GLFW_HEADER").unwrap_or("glfw.h".into());

    gen_bindings(&gl_header, "bindings_gl.rs"); // #include <GL/gl.h>
    gen_bindings(&glfw_header, "bindings_glfw.rs"); // #include <GLFW/glfw.h>
}

/// Generates bindings for a given header file.
///
/// This function takes a header file path and an output file name as arguments.
/// It uses the bindgen crate to generate Rust bindings for the specified header file.
/// The generated bindings are written to the specified output file.
///
/// # Arguments
///
/// * `header` - The path to the header file.
/// * `out_name` - The name of the output file.
///
fn gen_bindings(header: &str, out_name: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file = out_path.join(out_name);

    let bindings = bindgen::Builder::default()
        .header(header)
        .generate_comments(true)
        .clang_arg("-I/usr/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&out_file)
        .expect("Couldn't write bindings!");

    // Patches externs, dont remove.
    // For some reason, bindgen removed the ability to automatically mark externs
    // as unsafe, therefor breaking ANY extern functionality.
    //
    // No idea if mine is just outdated but its annoying asf.
    let contents = fs::read_to_string(&out_file).unwrap();
    let patched = format!(
        "#[allow(dead_code)]\n#[allow(unused_variables)]\n{}",
        contents
            .replace("extern \"C\"", "unsafe extern \"C\"")
            .replace("unsafe unsafe extern \"C\"", "unsafe extern \"C\"")
    );

    fs::write(&out_file, patched).unwrap();
}
