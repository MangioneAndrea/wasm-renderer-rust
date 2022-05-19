use crate::{log, log_i64};
use super::super::super::engine::raster;
use super::super::matrix;
use super::super::point;

pub struct Triangle {
    pub a: nalgebra::Vector2<f32>,
    pub b: nalgebra::Vector2<f32>,
    pub c: nalgebra::Vector2<f32>,
}


impl Triangle {
    pub fn draw(&self, raster: &mut raster::Raster) {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        if ab.x * ac.y - ab.y * ac.x > 0.0 {
            let mut m = nalgebra::Matrix2::new(ab.x, ab.y, ac.x, ac.y);
            m.try_inverse_mut();
            for y in 0..raster.height() {
                for x in 0..raster.width() {
                    let p = nalgebra::Vector2::new(x as f32, y as f32);
                    let uv = m * p;
                    //log(&(uv.x >= 0 && uv.y >= 0 && (uv.x + uv.y) < 1).to_string());
                    if uv.x >= 0.0 && uv.y >= 0.0 && (uv.x + uv.y) < 1.0 {
                        raster.set(
                            x,
                            y,
                            raster::Pixel {
                                r: 15,
                                g: 244,
                                b: 166,
                                depth: 3,
                            },
                        );
                    }
                }
            }
        }
    }
}
