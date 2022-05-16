use super::super::super::engine::raster;
use super::super::matrix;
use super::super::point;

pub struct Triangle {
    pub a: point::Point2D,
    pub b: point::Point2D,
    pub c: point::Point2D,
}




impl Triangle {
    pub fn draw(&self, raster: &mut raster::Raster) {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        if ab.x * ac.y - ab.y * ac.x > 0 {
            for y in 0..raster.height() {
                for x in 0..raster.width() {
                    let p = point::Point2D {
                        x: x as i64,
                        y: y as i64,
                    };
                    let m = matrix::Matrix2x2::new(ab, ac);
                    let uv = m * p;
                    if uv.x >= 0 && uv.y >= 0 && (uv.x + uv.y) < 1 {
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
