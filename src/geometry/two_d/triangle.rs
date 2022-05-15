use super::super::super::engine::ruster;
use super::super::point;
use crate::{HEIGHT, WIDTH};

pub struct Triangle<T> {
    pub a: point::Point3D<T>,
    pub b: point::Point3D<T>,
    pub c: point::Point3D<T>,
}

impl<T> Triangle<T> {
    pub fn Draw(&self, ruster: &ruster::Ruster) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {}
        }
    }
}
