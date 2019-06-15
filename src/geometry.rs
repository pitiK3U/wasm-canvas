use std::ops::{Div, Mul};

#[derive(Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }
}
pub fn distance(a: &Point, b: &Point) -> i32 {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let z = b.z - a.z;
    (((x * x) + (y * y) + (z * z)) as f32).sqrt() as i32
}

pub fn center(a: &Point, b: &Point) -> Point {
    Point {
        x: (a.x + b.x) / 2,
        y: (a.y + b.y) / 2,
        z: (a.z + b.z) / 2,
    }
}
#[derive(Debug, Default)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x, y, z }
    }

    pub fn from(a: &Point, b: &Point) -> Vector {
        Vector {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        }

    }

    pub fn size(&self) -> i32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt() as i32
    }

    pub fn normalize(self) -> Self {
        let size = self.size();
        self / size
    }
}

macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl Mul<Vector> for $t {
            type Output = Vector;

            fn mul(self, other: Vector) -> Vector {
                 Vector {x: (self * other.x as $t) as i32, y: (self * other.y as $t) as i32, z: (self * other.z as $t) as i32}
            }
        }

        impl Mul<$t> for Vector {
            type Output = Vector;

            fn mul(self, other: $t) -> Vector {
                 Vector {x: (self.x as $t * other) as i32, y: (self.y as $t * other) as i32, z: (self.z as $t * other) as i32}
            }
        }

        impl Div<$t> for Vector {
            type Output = Vector;

            fn div(self, other: $t) -> Vector {
                 Vector {x: (self.x as $t / other) as i32, y: (self.y as $t / other) as i32, z: (self.z as $t / other) as i32}
            }
        }
    )*)
 }

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use crate::geometry::Point;
    #[test]
    fn it_works() {
        let p = Point {
            ..Default::default()
        };
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.z, 0);
    }
}