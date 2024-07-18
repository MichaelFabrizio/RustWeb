// Build steps:
// 1) wasm-pack build --target web
// 2) python3 -m http.server
// 3) http://localhost:8000

pub mod context;
pub mod wasm_allocator;

extern crate core;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

/*
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
*/

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let header = document.create_element("header")?;
    let subheader = document.create_element("img")?;
    subheader.set_attribute("src", "/media/pro.jpg")?;

    header.append_child(&subheader)?;
    //    header.set_class_name("logo");
    //header.set_inner_html("<img src=\"/media/pro.jpg\" alt=\"Logo\">");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&header)?;
    body.append_child(&val)?;

    Ok(())
}
