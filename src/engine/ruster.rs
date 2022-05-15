use super::super::geometry::point;
use super::super::geometry::two_d::triangle;
use wasm_bindgen::prelude::*;

//use crate::{HEIGHT, SIZE, WIDTH};
#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    depth: u8,
}

#[wasm_bindgen]
pub struct Ruster {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Ruster {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Ruster {
        let mut pixels = Vec::new();
        pixels.resize(
            width * height,
            Pixel {
                r: 0,
                g: 0,
                b: 0,
                depth: 0xF,
            },
        );
        let t: triangle::Triangle<f64> = triangle::Triangle {
            a: point::Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            b: point::Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            c: point::Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        Ruster {
            pixels,
            width,
            height,
        }
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .map(|&pixel| vec![pixel.r, pixel.g, pixel.b, 0])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}
