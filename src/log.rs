use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` to bind `console.log(..)` instead of just `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  pub(crate) fn log(s: &str);

  // The `console.log` is quite polymorphic, so it can be bound with multiple signatures. Note that we need to use
  // `js_name` to ensure we always call `log`
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub(crate) fn log_u32(a: u32);

  // Log multiple arguments
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  pub(crate) fn log_many(a: &str, b: &str);
}
