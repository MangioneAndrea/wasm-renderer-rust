use crate::{log, log_i64};
use super::super::super::engine::raster;
use rand::Rng;
use super::super::matrix;
use super::super::point;

pub struct Triangle {
    pub a: nalgebra::Vector3<f32>,
    pub b: nalgebra::Vector3<f32>,
    pub c: nalgebra::Vector3<f32>,
    pub color: nalgebra::Vector3<u8>,
}


impl Triangle {
    pub fn new_random_color(a: nalgebra::Vector3<f32>, b: nalgebra::Vector3<f32>, c: nalgebra::Vector3<f32>) -> Triangle {
        let mut rng =rand::thread_rng();
        return Triangle::new(a, b, c, nalgebra::Vector3::new(rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()));
    }

    pub fn new(a: nalgebra::Vector3<f32>, b: nalgebra::Vector3<f32>, c: nalgebra::Vector3<f32>, color: nalgebra::Vector3<u8>) -> Triangle {
        return Triangle {
            a,
            b,
            c,
            color,
        };
    }


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
                                r: self.color.x,
                                g: self.color.y,
                                b: self.color.z,
                                depth: 3,
                            },
                        );
                    }
                }
            }
        }
    }
}
