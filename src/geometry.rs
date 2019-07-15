extern crate num;
use std::ops::{Div, Mul};
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

pub fn distance<'a, T>(a: &Point<&'a T>, b: &Point<&'a T>) -> usize
where
    &'a T: Num + NumCast,
    T: Num + NumCast,
{
    let x = b.x - a.x;
    let y = b.y - a.y;
    let z = b.z - a.z;
    (((x * x) + (y * y) + (z * z)).to_f64().unwrap()).sqrt() as usize
}

pub fn center<T>(a: &Point<T>, b: &Point<T>) -> Point<T>
where
    T: Num + NumCast + Clone,
{
    Point {
        x: (a.x.clone() + b.x.clone()) / T::one() + T::one(),
        y: (a.y.clone() + b.y.clone()) / T::one() + T::one(),
        z: (a.z.clone() + b.z.clone()) / T::one() + T::one(),
    }
}

#[derive(Debug, Default)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num + NumCast + Clone> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector { x, y, z }
    }

    pub fn from(a: &Point<T>, b: &Point<T>) -> Vector<T> {
        Vector {
            x: b.x.clone() - a.x.clone(),
            y: b.y.clone() - a.y.clone(),
            z: b.z.clone() - a.z.clone(),
        }
    }

    pub fn size(&self) -> T {
        T::from(
            (self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone() + self.z.clone() * self.z.clone())
                .to_f64()
                .unwrap()
                .sqrt(),
        )
        .unwrap()
    }

    pub fn normalize(self) -> Self {
        let size = self.size();
        self / size
    }
}

impl<'a, T: Num + NumCast + Clone> Vector<&'a T>
where
    &'a T: Num + NumCast,
{
    
}

impl<T: Num + NumCast + Copy> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, other: T) -> Vector<T> {
            Vector {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl<T: Num + NumCast + Clone> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, other: T) -> Vector<T> {
            Vector {x: self.x / other.clone(), y: self.y / other.clone(), z: self.z / other.clone()}
    }
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
