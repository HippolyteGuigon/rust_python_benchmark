use std::time::Instant;

fn compute(n: u128) -> u128 {
    let mut s: u128=0;
    for i in 0..n {
        s += i*i
    }
    s
}

fn main() {
    let n: u128=10000000;
    let start=Instant::now();
    let result=compute(n);
    let elapsed=start.elapsed();

    println!("Result: {}",result);
    println!("Elapsed (rust): {:?}",elapsed)
}