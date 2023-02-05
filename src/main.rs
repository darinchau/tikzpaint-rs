use tikzpaint_rs::figures::{Coordinates, Matrix, Concat, Projection};

fn main() {
    let x = Coordinates::new(&[3, 4, 5]);
    let proj1 = Matrix::new([
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 2]
    ]);
    let proj2 = Matrix::new([
        [1, 0, 2],
        [2, -1, 1]
    ]);
    let (proj3, _, _) = Concat::from(proj1, proj2);
    let y = proj3.call(&x);
}