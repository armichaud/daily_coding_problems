/*
 * Given integers M and N, write a program that counts how many positive integer pairs (a, b) satisfy the following conditions:
 * a + b = M
 * a XOR b = N 
 */

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: positive_integer_pairs <integer> <integer>");
        return;
    }
    let mut pairs = Vec::<(i32, i32)>::new();
    let m = args[1].parse::<i32>().expect("Must be an integer");
    let n = args[2].parse::<i32>().expect("Must be an integer");
    
    let half = m / 2;

    for a in 1..half {
        let b = m - a;
        if a ^ b == n {
            pairs.push((a, b));
        }
    }
    println!("Pairs: {:?}", pairs);
}
