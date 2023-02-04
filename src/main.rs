use tikzpaint_rs::figures::{Point, Figure, Coordinates};

fn main() {
    let p = Point::new(Coordinates::new(&[2, 3]));
    let p2 = Point::new(Coordinates::new(&[4, 5]));
    let fig = Figure::<2>::new();
}