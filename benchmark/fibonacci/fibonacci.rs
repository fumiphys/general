use std::time::Instant;

fn main(){
    let n = 40;
    let start = Instant::now();
    let fib = fibn(n);
    let end = start.elapsed();
    println!("rust {} {}sec.", fib, (end.as_secs() as f64) + (end.subsec_nanos() as f64) / 1000_000_000.);
}

fn fibn(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibn(n-1) + fibn(n-2)
    }
}
