mod math;
use crate::math::math::{center, distance, Point};

fn main() {
    let p1 = Point {
        ..Default::default()
    };
    let p2 = Point::new(1, 1, 1);
    let d = distance(&p1, &p2);
    let c = center(&p1, &p2);

    println!("{:?}", c);
}
