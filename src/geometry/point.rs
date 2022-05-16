use std::cmp;
use std::ops;

pub trait Num: PartialEq + num_traits::Num + std::cmp::Ord {}

#[derive(Eq, Copy, Clone)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, other: Point2D) -> Point2D {
        Point2D{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, other: Point2D) -> Point2D {
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<Point2D> for Point2D {
    type Output = Point2D;
    fn mul(self, other: Point2D) -> Point2D {
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Div<Point2D> for Point2D {
    type Output = Point2D;
    fn div(self, other: Point2D) -> Point2D {
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl cmp::Ord for Point2D {
    fn cmp(&self, other: &Point2D) -> cmp::Ordering {
        if self.x > other.x {
            return cmp::Ordering::Greater;
        } else if self.x < other.x {
            return cmp::Ordering::Less;
        } else if self.y > other.y {
            return cmp::Ordering::Greater;
        } else if self.y < other.y {
            return cmp::Ordering::Less;
        }
        cmp::Ordering::Equal
    }
}
impl cmp::PartialEq for Point2D {
    fn eq(&self, other: &Point2D) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl cmp::PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Point2D) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ops::Rem<Point2D> for Point2D {
    type Output = Point2D;
    fn rem(self, other: Point2D) -> Point2D {
        Point2D{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl num_traits::Zero for Point2D {
    fn zero() -> Point2D {
        Point2D{
            x: 0,
            y: 0,
        }
    }
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

pub struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}
impl ops::Add<Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, other: Point3D) -> Point3D {
        Point3D{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl ops::Sub<Point3D> for Point3D {
    type Output = Point3D;
    fn sub(self, other: Point3D) -> Point3D {
        Point3D{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Point3D {
    pub fn as2d(self) -> Point2D {
        Point2D{
            x: self.x,
            y: self.y,
        }
    }
}
