use rand::Rng;
use std::thread;
use std::time::Instant;

pub fn estimating_pi(total: u32) -> f64 {
    let num_cpus = num_cpus::get() as u32;

    let mut handles = vec![];

    for _ in 0..num_cpus {
        handles.push(thread::spawn(move || {
            let mut approximation = 0u64;
            let mut rng = rand::thread_rng();
            for _ in 0..total {
                let a = rng.gen::<f64>();
                let b = rng.gen::<f64>();
                let c = a.powf(2f64) + b.powf(2f64);
                if c <= 1f64 {
                    approximation += 1;
                }
            }
            approximation
        }))
    }

    let mut approximation = 0u64;
    for handle in handles {
        approximation += handle.join().unwrap();
    }
    let pi = approximation as f64 / (num_cpus as f64 * total as f64) * 4.0;
    pi
}

fn main() {
    println!("Hello, world!");
    let now = Instant::now();
    let pi = estimating_pi(1_000_000);
    println!("{:.2} secs", now.elapsed().as_secs_f64());
    println!("an approximation of the pi is {}", pi);
    println!("Constant std::f32::consts::PI is {}", std::f32::consts::PI);
}
