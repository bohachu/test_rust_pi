use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let pi = chudnovsky(50);

    println!("Pi = {:.50}", pi);
    println!("Calculation time: {:.2?}", start_time.elapsed());
}

fn chudnovsky(digits: i32) -> f64 {
    let k = ((digits as f64) / 14.181647462725477).ceil() as i32;
    let mut sum = 0.0;
    for i in 0..k {
        let num = factorial(6 * i) as f64 * (13591409 + 545140134 * i) as f64;
        let denom = factorial(3 * i) as f64 * (factorial(i) as f64).powi(3) * (-262537412640768000).pow(i);
        sum += num / denom;
    }
    let pi = 426880.0 * ((10005.0_f64).sqrt() / sum);
    (pi * 10_f64.powi(digits - 1)).round() / 10_f64.powi(digits - 1)
}

fn factorial(n: i32) -> i32 {
    (1..=n).product()
}
