/*
    Given a positive integer N, find the smallest number of steps it will take to reach 1.

    There are two kinds of permitted steps:

    You may decrement N to N - 1.
    If a * b = N, you may decrement N to the larger of a and b.
    For example, given 100, you can reach 1 in five steps with the following route: 100 -> 10 -> 9 -> 3 -> 2 -> 1.
*/

fn main() {
    let n = 100;
    let mut path = Path::new(vec![n]);
    path.flesh_out();
    println!("{}: {:?}, {} steps", n, path.steps, path.steps.len());
}

struct Path {
    steps: Vec<u64>,
}

impl Path {
    fn new(path: Vec<u64>) -> Path {
        Path { steps: path }
    }

    fn get_last(&self) -> u64 {
        *self.steps.last().unwrap()
    }

    fn get_shortest_path_from_factors(&mut self) -> Vec<u64> {
        let factors = Path::get_factors(self.get_last());
        let mut shortest_path = Vec::new();

        for (_, b) in factors {
            let mut new_path = Path::new(self.steps.clone());
            new_path.steps.push(b);
            new_path.flesh_out();
            if shortest_path.len() == 0 || new_path.steps.len() < shortest_path.len() {
                shortest_path = new_path.steps;
            }
        }
        shortest_path
    }

    fn flesh_out(&mut self) {
        let last = self.get_last();
        if last == 1 {
            return;
        }
        
        let mut decremented = Path::new(self.steps.clone());
        decremented.steps.push(last - 1);
        decremented.flesh_out();
        
        let shortest_factor_path = self.get_shortest_path_from_factors();

        if shortest_factor_path.len() == 0 || decremented.steps.len() < shortest_factor_path.len() {
            self.steps = decremented.steps;
        } else {
            self.steps = shortest_factor_path;
        }
    }

    fn get_factors(n: u64) -> Vec<(u64, u64)> {
        let mut factors = Vec::new();
        let half = n / 2;
        let mut i = 2;
        while i <= half {
            if n % i == 0 {
                factors.push((i, n / i));
            }
            i += 1;
        }
        factors.into_iter().map(|(a, b)| {
            if a > b {
                (b, a)
            } else {
                (a, b)
            }
        }).collect()
    }
}
