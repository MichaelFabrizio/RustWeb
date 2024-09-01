// Build steps:
// 1) wasm-pack build --target web
// 2) python3 -m http.server
// 3) http://localhost:8000

pub(crate) mod context;

#[macro_use]
pub(crate) mod wasm_allocator;
pub(crate) mod web_core;

extern crate core;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::web_core::WebCore;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
}

/*
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
*/

struct TestObject {
    a: i64,
}

// BEGIN WEBGL PROGRAM LINKING EXAMPLE
// AND SHADER COMPILATION FUNCTIONS
// URL: https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = (((amt as u64) % 1_000) as u32) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}

// END WEBGL CODE EXAMPLE SNIPPET

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let performance = window
        .performance()
        .expect("performance should be available");

    console_log!("the current time (in ms) is {}", performance.now());

    let mut webcore: WebCore = WebCore::new();
    webcore.addkeyvec::<TestObject, u16, 4000>();

    // BEGIN WEBGL CODE EXAMPLE SNIPPET
    // URL: https://rustwasm.github.io/docs/wasm-bindgen/examples/webgl.html
    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        r##"#version 300 es
 
        in vec4 position;

        void main() {
        
            gl_Position = position;
        }
        "##,
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        r##"#version 300 es
    
        precision highp float;
        out vec4 outColor;
        
        void main() {
            outColor = vec4(1, 1, 1, 1);
        }
        "##,
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(
        position_attribute_location as u32,
        3,
        WebGl2RenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let vert_count = (vertices.len() / 3) as i32;
    draw(&context, vert_count);

    // END WEBGL CODE EXAMPLE SNIPPET

    // STEP 1: PARSING EXISTENT HTML BLOCKS
    // POSSIBILITY A) 'VALID' HTML CONSTRUCT
    // POSSIBILITY B) 'INVALID' HTML CONSTRUCT
    //
    // Goal: Not all user defined HTML can be regarded as valid within the scope of this program.
    // Because we will be building a component model, there is a translational layer which converts between the
    // HTML DOM to VIRTUAL DOM to the COMPONENT HIERARCHY
    //
    // Example:
    // An entity ID = 1 could have a Nav component attached, which implies it is nested within a
    // <nav> block.

    //    let header = document.create_element("header")?;
    //    let subheader = document.create_element("img")?;
    //    subheader.set_attribute("src", "/media/pro.jpg")?;

    //    header.append_child(&subheader)?;
    //    header.set_class_name("logo");
    //header.set_inner_html("<img src=\"/media/pro.jpg\" alt=\"Logo\">");

    // Manufacture the element we're gonna append
    //let val = document.create_element("h1")?;
    //val.set_inner_html("Hello from Rust Test Change!");

    //    body.append_child(&header)?;
    //body.append_child(&val)?;

    console_log!("the current time (in ms) is {}", performance.now());
    Ok(())
}

/* WEBSYS CONSOLE LOGGING FUNCTIONALITY!!!!
* TAKES JsValue as parameters
*
fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}
*/
