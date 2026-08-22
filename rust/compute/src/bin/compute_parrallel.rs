use rayon::prelude::*;
use std::time::Instant;

fn compute(n: u128) -> u128 {
    (0..n).map(|i| i * i).sum()
}

fn main() {
    let n=10000000;
    let workers=num_cpus::get();

    let start=Instant::now();

    let result: u128 = (0..workers).into_par_iter().map(|_| compute(n)).sum();
    let elapsed=start.elapsed();

    println!("Result: {}", result);
    println!("Elapsed (rust parallel): {:?}", elapsed);

}