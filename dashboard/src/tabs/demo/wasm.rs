// what: wasm measurement utilities and helper functions
// why: provides real webassembly api timing for demo and proof tabs
// relations: used by component.rs, could be shared with proof.rs in future

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// ============================================================================
// wasm measurement (real webassembly api calls)
// ============================================================================

/// minimal valid wasm module for instantiation timing
pub const MINIMAL_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, // magic
    0x01, 0x00, 0x00, 0x00, // version
    0x01, 0x07, 0x01, 0x60, 0x02, 0x7f, 0x7f, 0x01, 0x7f, // type section
    0x03, 0x02, 0x01, 0x00, // function section
    0x07, 0x07, 0x01, 0x03, 0x61, 0x64, 0x64, 0x00, 0x00, // export section
    0x0a, 0x09, 0x01, 0x07, 0x00, 0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b, // code
];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    pub fn now() -> f64;
    
    // Run Python code via Pyodide
    // Note: pyodideReady and pyodideLoadTime accessed via js_sys::Reflect
    // to avoid deprecated JsStatic warnings
    #[wasm_bindgen(catch, js_namespace = window)]
    pub async fn runPython(code: &str) -> Result<JsValue, JsValue>;
}

/// measure wasm instantiation time (averaged over 10 iterations)
pub async fn measure_instantiate_time() -> f64 {
    let array = js_sys::Uint8Array::from(MINIMAL_WASM);
    let compile_promise = js_sys::WebAssembly::compile(&array.buffer());
    let module: js_sys::WebAssembly::Module = wasm_bindgen_futures::JsFuture::from(compile_promise)
        .await
        .unwrap()
        .unchecked_into();
    
    // Run 10 iterations and average for more accurate sub-millisecond timing
    let iterations = 10;
    let start = now();
    
    for _ in 0..iterations {
        let instantiate_promise = js_sys::WebAssembly::instantiate_module(
            &module,
            &js_sys::Object::new()
        );
        let _ = wasm_bindgen_futures::JsFuture::from(instantiate_promise).await;
    }
    
    (now() - start) / iterations as f64
}

// ============================================================================
// helper functions
// ============================================================================

/// set a timeout callback with the given duration
pub fn set_timeout<F: FnOnce() + 'static>(cb: F, dur: std::time::Duration) {
    use wasm_bindgen::closure::Closure;
    let window = web_sys::window().unwrap();
    let closure = Closure::once(cb);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(), dur.as_millis() as i32
    ).unwrap();
    closure.forget();
}
