use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
  bare_bones();
  using_a_macro();
  using_web_sys();
}

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` to bind `console.log(..)` instead of just `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so it can be bound with multiple signatures. Note that we need to use
  // `js_name` to ensure we always call `log`
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Log multiple arguments
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

fn bare_bones() {
  log("Hi from Rust");
  log_u32(4);
  log_many("Logging", "many values");
}

// Define macro to get `println!` functionality for `console.log`
macro_rules! console_log {
  // Using `log` function imported in `bare_bones` function
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
  console_log!("Hello {}!", "t420babe");
  console_log!("Let's print some numbers");
  console_log!("2 + 2 = {}", 2 + 2);
}

fn using_web_sys() {
  use web_sys::console;

  console::log_1(&"Hello using web-sys".into());

  let js: JsValue = 4.into();
  console::log_2(&"Logging arbitrary values looks like".into(), &js);
}
