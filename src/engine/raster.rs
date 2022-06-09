use super::super::geometry::point;
use super::super::geometry::two_d::triangle;
use super::super::geometry::three_d::cube;
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
pub struct Raster {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

#[wasm_bindgen]
impl Raster {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Raster {
        let mut pixels = Vec::new();
        pixels.resize(
            width * height,
            Pixel {
                r: 255,
                g: 255,
                b: 255,
                depth: 255,
            },
        );
        let t: triangle::Triangle = triangle::Triangle::new_random_color(
            nalgebra::Vector3::new(0.0, 0.0, 0.0),
            nalgebra::Vector3::new(1500.0, 400.0, 0.0),
            nalgebra::Vector3::new(1500.0, 1500.0, 0.0),
        );

        let o: triangle::Triangle = triangle::Triangle::new_random_color(nalgebra::Vector3::new(0.0, 0.0, 0.0),
                                                                         nalgebra::Vector3::new(0.0, 1500.0, 0.0),
                                                                         nalgebra::Vector3::new(1500.0, 1500.0, 0.0),
        );

        let c = cube::Cube::new(
            nalgebra::Vector3::new(500.0, 500.0, 0.0),
            nalgebra::Vector3::new(500.0, 500.0, 0.0),
        );

        let mut raster = Raster {
            pixels,
            width,
            height,
        };

        o.draw(&mut raster);
        t.draw(&mut raster);

        c.draw(&mut raster);


        raster
    }

    pub fn pixels(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .map(|&pixel| vec![pixel.r, pixel.g, pixel.b, 255])
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

impl Raster {
    pub fn set(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y * self.width + x] = pixel;
    }
}
