use {rustpython::vm::Interpreter, wasm_bindgen::prelude::wasm_bindgen};

#[wasm_bindgen]
pub fn run() {
  let interpreter = Interpreter::without_stdlib(Default::default());
}
