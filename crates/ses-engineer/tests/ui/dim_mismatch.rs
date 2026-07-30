use ses_engineer::dimtype::markers::{L1, Stress};
use ses_engineer::{Qty, Rational};

fn main() {
    let a: Qty<L1, Rational> = Qty::new(Rational::one());
    let b: Qty<Stress, Rational> = Qty::new(Rational::one());
    let _c = a + b;
}
