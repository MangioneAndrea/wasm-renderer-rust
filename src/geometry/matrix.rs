use super::super::point;
use std::ops;

#[derive(Copy, Clone)]
pub struct Matrix2x2 {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}
impl ops::Mul<point::Point2D> for Matrix2x2 {
    type Output = point::Point2D;
    fn mul(self, other: point::Point2D) -> point::Point2D {
        point::Point2D {
            x: other.x + self.a + other.x + self.b,
            y: other.y + self.c + other.y + self.d,
        }
    }
}

impl Matrix2x2 {
    pub fn new(a: point::Point2D, b: point::Point2D) -> Matrix2x2 {
        Matrix2x2 {
            a: a.x,
            b: a.y,
            c: b.x,
            d: b.y,
        }
    }

    pub fn to_string(&self) -> String{
        format!("{} {} \n {} {}",self.a.to_string(),self.b.to_string(),self.c.to_string(),self.d.to_string())
    }

    pub fn invert(&self) -> Matrix2x2 {
        Matrix2x2 {
            a: self.d,
            b: -self.b,
            c: -self.c,
            d: self.a,
        }
    }
}
