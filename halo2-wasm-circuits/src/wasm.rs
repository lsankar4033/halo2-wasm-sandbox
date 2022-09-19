use wasm_bindgen::prelude::*;

use crate::fibonacci::FiboCircuit;

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     log(name)
// }

#[wasm_bindgen]
pub fn prove_fib() {
    // TODO: figure out inputs, etc.
    let circuit = FiboCircuit::<Fp> {};
}
