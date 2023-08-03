use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("Usage: cargo run <stream...>");
        return;
    }
    let stream = args[1..].iter().map(|s| s.parse::<f32>().expect("stream must contain only integers"));
    let mut median_calculator = MedianCalculator::new();
    for n in stream {
        median_calculator.insert(n);
        println!("{}", median_calculator.get_median());
    }
}

struct MedianCalculator {
    sequence: Vec<f32>
}

impl MedianCalculator {
    fn new() -> MedianCalculator {
        MedianCalculator { sequence: Vec::new() }
    }

    fn insert(&mut self, n: f32) {
        // Iterates through sequence and inserts n in the correctly sorted position
        let mut i = 0;
        while i < self.sequence.len() && self.sequence[i] < n {
            i += 1;
        }
        self.sequence.insert(i, n);   
    }

    fn get_median(&self) -> f32 {
        let length = self.sequence.len();
        let i = length / 2;
        if length % 2 == 0 {
            let j = i - 1;
            return (self.sequence[i] + self.sequence[j]) / 2.0;
        }
        return self.sequence[i];
    }
}