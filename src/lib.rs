mod utils;
mod geometry {
    pub mod point;
    pub mod matrix;
    pub mod two_d {
        pub mod triangle;
    }
}
mod engine {
    pub mod ruster;
}
use engine::ruster::Ruster;
use geometry::point;
use geometry::two_d::triangle;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 800;
const SIZE: usize = (HEIGHT * WIDTH) as usize;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen(start)]
pub fn start() {

}
