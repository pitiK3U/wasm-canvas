//extern crate num;
//use num::{Num, NumCast};
use std::ops::{Add, Div, Mul, Sub};

use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

/// The basic scalar type for all structures of `nalgebra`.
///
/// This does not make any assumption on the algebraic properties of `Self`.
pub trait Scalar:
    Copy
    + PartialEq
    + Debug
    + Any
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Add<Output = Self>
    + Div<Output = Self>
{
    #[inline]
    /// Tests if `Self` the same as the type `T`
    ///
    /// Typically used to test of `Self` is a f32 or a f64 with `N::is::<f32>()`.
    fn is<T: Scalar>() -> bool {
        TypeId::of::<Self>() == TypeId::of::<T>()
    }
}
impl<
        T: Copy
            + PartialEq
            + Debug
            + Any
            + Sub<Output = Self>
            + Mul<Output = Self>
            + Add<Output = Self>
            + Div<Output = Self>,
    > Scalar for T
{
}

/// ```
/// #use super::*;
/// assert_eq!(Point{..Default::default()}, Point::new(0,0,0));
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Point<T> {
        Point { x, y, z }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }
}

pub fn distance<'a, T>(a: &'a Point<T>, b: &'a Point<T>) -> usize
where
    // &'a T: Scalar,
    T: Scalar,
    f64: std::convert::From<T>,
{
    let x = b.x - a.x;
    let y = b.y - a.y;
    let z = b.z - a.z;
    (f64::from(((x * x) + (y * y) + (z * z)))).sqrt() as usize
}

pub fn center<T>(a: &Point<T>, b: &Point<T>) -> Point<T>
where
    T: Scalar + std::convert::From<i8>,
{
    Point {
        x: (a.x.clone() + b.x.clone()) / T::from(2),
        y: (a.y.clone() + b.y.clone()) / T::from(2),
        z: (a.z.clone() + b.z.clone()) / T::from(2),
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Vector<T> {
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

    pub fn size(&self) -> T
    where
        f64: std::convert::From<T>,
        T: std::convert::From<f64>,
    {
        T::from(
            f64::from(
                self.x.clone() * self.x.clone()
                    + self.y.clone() * self.y.clone()
                    + self.z.clone() * self.z.clone(),
            )
            .sqrt(),
        )
    }

    pub fn normalize(self) -> Self
    where
        f64: std::convert::From<T>,
        T: std::convert::From<f64>,
    {
        let size = self.size();
        self / size
    }
}

/* impl<'a, T: Num + NumCast + Clone> Vector<&'a T> where &'a T: Num + NumCast {} */

impl<T: Scalar> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, other: T) -> Vector<T> {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/* impl<T: Num + NumCast + Copy> Mul<Vector<T>> for T {
    type Output = Vector<T>;

    fn mul(self, other: Vector<T>) -> Vector<T> {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
} */

impl<T: Scalar> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, other: T) -> Vector<T> {
        Vector {
            x: self.x / other.clone(),
            y: self.y / other.clone(),
            z: self.z / other.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_point() {
        let p: Point<i32> = Point {
            ..Default::default()
        };
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 0);
        assert_eq!(p.z, 0);
    }

    #[test]
    fn point_eq() {
        let p1: Point<i32> = Point {
            ..Default::default()
        };
        let p2 = Point::new(0, 0, 0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn distance() {
        let p1 = Point::default();
        let p2 = Point::new(1, 1, 1);
        let dist = super::distance(&p1, &p2);
        assert_eq!(dist, 1_usize);
    }

    #[test]
    fn center() {
        let p1 = Point::new(0f32, 0f32, 0f32);
        let p2 = Point::new(1f32, 1f32, 1f32);
        let dist = super::center(&p1, &p2);
        assert_eq!(dist, Point::new(0.5, 0.5, 0.5));
    }
    #[test]
    fn vector_default() {
        let v: Vector<i32> = Vector::default();
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
        assert_eq!(v.z, 0);
    }

    #[test]
    fn vector_new() {
        let v1: Vector<i32> = Vector::default();
        let v2 = Vector::new(0, 0, 0);
        assert_eq!(v1, v2);
    }
}
