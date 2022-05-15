mod utils;
mod geometry {
    pub mod point;
    pub mod two_d {
        pub mod triangle;
    }
}
use geometry::point;
use geometry::two_d::triangle;

use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const WIDTH: f64 = 1600.0;
const HEIGHT: f64 = 1600.0;

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
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let t: triangle::Triangle<f64> = triangle::Triangle {
        a: point::Point2D { x: 0.0, y: 0.0 },
        b: point::Point2D { x: 0.0, y: 0.0 },
        c: point::Point2D { x: 0.0, y: 0.0 },
    };
    ctx.begin_path();
    ctx.move_to(10.0, 10.0);
    ctx.line_to(100.0, 50.0);
    ctx.line_to(50.0, 100.0);
    ctx.line_to(0.0, 90.0);
    ctx.close_path();
    ctx.stroke();
}
