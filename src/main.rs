use tikzpaint_rs::figures::{Point, Coordinates, Figure, Identity};

fn main() {
    let p1 = Point::new(Coordinates::new([2, 3]));
    let p2 = Point::new(Coordinates::new([4, 5]));
    let mut fig = Figure::<2>::new();
    fig.draw(&p1);
    fig.draw(&p2);

    let result = fig.output_tikz(&Identity);
    println!("{}", result);
}