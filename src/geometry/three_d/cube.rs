use crate::engine::raster;
use crate::geometry::two_d::triangle;

pub struct Cube {
    center: nalgebra::Vector3<f32>,
    rotation: nalgebra::Vector3<f32>,
}

impl Cube {
    pub fn new(c: nalgebra::Vector3<f32>, r: nalgebra::Vector3<f32>) -> Self {
        Cube {
            center: c,
            rotation: r,
        }
    }
    pub fn draw(&self, raster: &mut raster::Raster) {
        let v = [
            nalgebra::Vector3::new(-1.0, -1.0, -1.0),
            nalgebra::Vector3::new(1.0, -1.0, -1.0),
            nalgebra::Vector3::new(1.0, 1.0, -1.0),
            nalgebra::Vector3::new(-1.0, 1.0, -1.0),
            nalgebra::Vector3::new(-1.0, -1.0, 1.0),
            nalgebra::Vector3::new(1.0, -1.0, 1.0),
            nalgebra::Vector3::new(1.0, 1.0, 1.0),
            nalgebra::Vector3::new(-1.0, 1.0, 1.0)
        ];
        let triangles = [
            triangle::Triangle { a: v[0], b: v[1], c: v[2] },
            triangle::Triangle { a: v[0], b: v[2], c: v[3] },
            triangle::Triangle { a: v[7], b: v[6], c: v[5] },
            triangle::Triangle { a: v[7], b: v[5], c: v[4] },
            triangle::Triangle { a: v[0], b: v[3], c: v[7] },
            triangle::Triangle { a: v[0], b: v[7], c: v[4] },
            triangle::Triangle { a: v[2], b: v[1], c: v[5] },
            triangle::Triangle { a: v[2], b: v[5], c: v[6] },
            triangle::Triangle { a: v[3], b: v[2], c: v[6] },
            triangle::Triangle { a: v[3], b: v[6], c: v[7] },
            triangle::Triangle { a: v[1], b: v[0], c: v[4] },
            triangle::Triangle { a: v[1], b: v[4], c: v[5] },
        ];

        for x in triangles {
            x.draw(raster);
        }
    }
}