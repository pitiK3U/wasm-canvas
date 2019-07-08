extern crate num;
use num::{Num, NumCast};

/// ```
/// #use super::*;
/// assert_eq!(Point{..Default::default()}, Point::new(0,0,0));
/// ```
#[derive(Debug, Default)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }
}

pub fn distance<T: Num + NumCast>(a: &Point<T>, b: &Point<T>) -> usize {
    let x = b.x - a.x;
    let y = b.y - a.y;
    let z = b.z - a.z;
    (((x * x) + (y * y) + (z * z)).to_f64().unwrap()).sqrt() as usize
}

pub fn center<T: Num>(a: &Point<T>, b: &Point<T>) -> Point<T> {
    let two = T::one() + T::one();
    Point {
        x: (a.x + b.x) / two,
        y: (a.y + b.y) / two,
        z: (a.z + b.z) / two,
    }
}

#[derive(Debug, Default)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + NumCast> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector { x, y, z }
    }

    pub fn from(a: &Point<T>, b: &Point<T>) -> Vector<T> {
        Vector {
            x: b.x - a.x,
            y: b.y - a.y,
            z: b.z - a.z,
        }

    }

    pub fn size(&self) -> T {
        T::from( (self.x * self.x + self.y * self.y + self.z * self.z).to_f64().unwrap().sqrt() ).unwrap()
    }

    /* pub fn normalize(self) -> Self {
        let size = self.size();
        self / size
    } */
}
/*
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
}*/
