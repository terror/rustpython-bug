use {
  rustpython::vm::Interpreter,
  wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast, JsValue},
};

#[wasm_bindgen]
pub fn run() {
  let interpreter = Interpreter::without_stdlib(Default::default());
}
