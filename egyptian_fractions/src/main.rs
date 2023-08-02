use num_rational::Rational32 as Rational;

fn main() {
    let mut fraction = Rational::new(4, 13);
    let mut egyptian = Vec::<Rational>::new();
    while fraction != Rational::new(0, 1) {
        let mut denom = 2;
        while fraction < Rational::new(1, denom) {
            denom += 1;
        }
        egyptian.push(Rational::new(1, denom));
        fraction = fraction - Rational::new(1, denom);
    }
    println!("{:?}", egyptian);
}
