use std::env;
use num_rational::Rational32 as Rational;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if at least two arguments were provided
    if args.len() < 3 {
        println!("Usage: cargo run <numerator> <denominator>");
        return;
    }

    let arg1 = &args[1];
    let arg2 = &args[2];

    let numerator = arg1.parse::<i32>().expect("Numerator must be an integer");
    let denominator = arg2.parse::<i32>().expect("Denominator must be an integer");    

    let mut fraction = Rational::new(numerator, denominator);
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
