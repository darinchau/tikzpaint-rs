//! Projections are traits that takes coordinates and outputs coordinates

// use std::ops::Mul;

use crate::figures::Coordinates;

pub struct Projection<const INPUT: usize, const OUTPUT: usize> {
    f: Box<dyn Fn(&Coordinates<INPUT>) -> Coordinates<OUTPUT>>
}

impl<const INPUT: usize, const OUTPUT: usize> Projection<INPUT, OUTPUT> {
    fn call(&self, v: &Coordinates<INPUT>) -> Coordinates<OUTPUT> {
        return (self.f)(v);
    }
}

// impl<'a, const INPUT: usize, const i: usize, const OUTPUT: usize> Mul<Projection<i, OUTPUT>> for &'a Projection<INPUT, i> {
//     type Output = Projection<INPUT, OUTPUT>;
//     fn mul(self, rhs: Projection<i, OUTPUT>) -> Projection<INPUT, OUTPUT> {
//         Projection {
//             f: Box::new(|x| {
//                 let y = self.call(x);
//                 rhs.call(&y)
//             })
//         }
//     }
// }