mod geometry;
use crate::geometry::{center, distance, Point, Vector};

fn main() {
    let p1 = &Point {
        ..Default::default()
    };
    let p2 = &Point::new(1, 2, 3);
    let d = distance(p1, p2);
    let c = center(p1, p2);
    let v = Vector::from(p1, p2);

    println!("{:?}", v.normalize());
}
