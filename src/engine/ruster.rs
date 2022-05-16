use super::super::geometry::point;
use super::super::geometry::two_d::triangle;
use wasm_bindgen::prelude::*;

//use crate::{HEIGHT, SIZE, WIDTH};
#[derive(Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub depth: u8,
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
        let t: triangle::Triangle = triangle::Triangle {
            a: point::Point2D { x: 0, y: 0 },
            b: point::Point2D { x: 150, y: 120 },
            c: point::Point2D { x: 233, y: 75 },
        };

        let ruster = Ruster {
            pixels,
            width,
            height,
        };

        t.draw(&ruster);

        ruster
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
impl Ruster {
    pub fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y * self.width + x] = pixel;
    }
}
