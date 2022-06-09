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
            triangle::Triangle::new_random_color(v[0], v[1], v[2]),
            triangle::Triangle::new_random_color(v[0], v[2], v[3]),
            triangle::Triangle::new_random_color(v[7], v[6], v[5]),
            triangle::Triangle::new_random_color(v[7], v[5], v[4]),
            triangle::Triangle::new_random_color(v[0], v[3], v[7]),
            triangle::Triangle::new_random_color(v[0], v[7], v[4]),
            triangle::Triangle::new_random_color(v[2], v[1], v[5]),
            triangle::Triangle::new_random_color(v[2], v[5], v[6]),
            triangle::Triangle::new_random_color(v[3], v[2], v[6]),
            triangle::Triangle::new_random_color(v[3], v[6], v[7]),
            triangle::Triangle::new_random_color(v[1], v[0], v[4]),
            triangle::Triangle::new_random_color(v[1], v[4], v[5]),
        ];

        for x in triangles {
            x.draw(raster);
        }
    }
}