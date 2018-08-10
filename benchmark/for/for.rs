use std::time::Instant;

fn main(){
    let n = 1000000;
    let start = Instant::now();
    let mut sum: u64 = 0;
    for i in 1..n {
        sum += i;
    }
    let end = start.elapsed();
    println!("rust {} {}sec.", sum, (end.as_secs() as f64) + (end.subsec_nanos() as f64) / 1000_000_000.);
}
