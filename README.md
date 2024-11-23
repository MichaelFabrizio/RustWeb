Opens the general coverage of documentation for a new type of web framework.

***
## Purpose: To deliver a unified Rust / Webassembly deployment to enable scalable 3D graphics within a static HTML context.

## Building / Deploying locally:
First clone the repository:
```
git clone git@github.com:MichaelFabrizio/RustWeb.git
```

'cd' into your repository:
```
cd RustWeb
```

From within your cloned repository:
``` Rust
wasm-pack build --target=web
python3 -m http.server
```

Then visit the  in a browser of your choice, example:
```
localhost:8000/
```
The address may depend on your python3 http.server configuration.

## Important items:
**WebCore** - Responsible for the memory management and data composition. It holds a few key items:

**WasmAllocator** - A memory allocator which holds blocks of KeyVector data. Created implicitly by the WebCore struct.

**KeyVector** - Represents a block of memory that is held within the WasmAllocator instance. Each block is constructed with the addkeyvec<T, I, N>() function. 

**T: Type** - A generic, user-defined data type which is defined as a struct of fundamental types.

**I: IndexType** - Contains sequencing and algorithmic encoding for representing data.

**N: Number** - The _array quantity of elements of type T_.

**Example of a KeyVector:**

Suppose we wish to represent data of type A, with a given quantity N:
``` Rust
// A is a composite of other types which are themselves fundamental data.
struct A {
typeB: B,
typeC: C, 
}
```

Then within the main() loop, the user may call:
``` Rust
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Begin: Default initialization logic
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    // End: Default initialization logic

    // User creates an instance of WebCore
    let mut webcore: WebCore = WebCore::new();

    // User creates an instance of KeyVector, which is of type A, and holds 4000 elements.
    let test_keyvec = webcore.addkeyvec::<A, u16, 4000>();
```
***
