# gl_bindings
i got tired of using the default libraries for opengl, so i just built them myself.

To use, put `gl_bindings = { git = "https://github.com/Zephyros1938/gl_bindings.git" }` in the dependencies table of your Cargo.toml file.

## Usage

To use gl_bindings, just import:

```rust
use gl_bindings::*;
```

## Examples

### Creating a Window

```rust
use gl::*;
use gl_bindings::{gl, glfw};
use glfw::{
    GLFW_CONTEXT_VERSION_MAJOR, GLFW_CONTEXT_VERSION_MINOR, GLFW_OPENGL_CORE_PROFILE,
    GLFW_OPENGL_PROFILE, glfwCreateWindow, glfwDestroyWindow, glfwInit, glfwMakeContextCurrent,
    glfwPollEvents, glfwSwapBuffers, glfwTerminate, glfwWindowHint, glfwWindowShouldClose,
};

use std::ffi::CString;
use std::ptr;

fn main() {
    unsafe {
        if glfwInit() == 0 {
            panic!("GLFW failed to initialize!");
        }

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR as i32, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR as i32, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE as i32, GLFW_OPENGL_CORE_PROFILE as i32);

        let width: i32 = 800;
        let height: i32 = 600;
        let title = CString::new("My GLFW Window").unwrap();

        let window = glfwCreateWindow(
            width,
            height,
            title.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        if window.is_null() {
            glfwTerminate();
            panic!("Failed to create GLFW window!");
        }

        glfwMakeContextCurrent(window);

        while glfwWindowShouldClose(window) == 0 {
            glfwPollEvents();

            glClearColor(0.1, 0.2, 0.3, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);

            glfwSwapBuffers(window);
        }

        glfwDestroyWindow(window);
        glfwTerminate();
    }
}
```
