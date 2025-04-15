#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_variables
)]
pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings_gl.rs"));
}

#[allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_variables
)]
pub mod glfw {
    include!(concat!(env!("OUT_DIR"), "/bindings_glfw.rs"));
}
