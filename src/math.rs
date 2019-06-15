pub mod math {
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
        (((x * x) + (y * y) + (z * z)) as f64).sqrt() as i32
    }

    pub fn center(a: &Point, b: &Point) -> Point {
        Point {
            x: (a.x + b.x) / 2,
            y: (a.y + b.y) / 2,
            z: (a.z + b.z) / 2,
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::math::math::Point;
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
}
